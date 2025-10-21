use crate::errors::PencilError;
use anchor_lang::prelude::*;

/// 计算利息
pub fn calculate_interest(principal: u64, rate: u16, periods: u64) -> Result<u64> {
    let rate_decimal = (rate as u128) / 10000u128;
    let interest = ((principal as u128)
        .checked_mul(rate_decimal)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_mul(periods as u128)
        .ok_or(PencilError::ArithmeticOverflow)?) / 365u128;
    Ok(interest as u64)
}

/// 计算费用
pub fn calculate_fee(amount: u64, fee_rate: u16) -> Result<u64> {
    let fee = ((amount as u128)
        .checked_mul(fee_rate as u128)
        .ok_or(PencilError::ArithmeticOverflow)?) / 10000u128;
    Ok(fee as u64)
}

/// 计算当前还款期数
pub fn calculate_current_period(
    funding_end_time: i64,
    repayment_period: u64,
    current_time: i64,
) -> Result<u64> {
    if current_time <= funding_end_time {
        return Ok(0);
    }

    let elapsed = (current_time - funding_end_time) as u64;
    let period_seconds = repayment_period.checked_mul(86400).ok_or(PencilError::ArithmeticOverflow)?;
    let period = elapsed.checked_div(period_seconds).ok_or(PencilError::ArithmeticOverflow)?;

    Ok(period)
}

/// 验证 Junior 占比
pub fn validate_junior_ratio(junior_amount: u64, total_amount: u64, min_ratio: u16) -> Result<()> {
    if total_amount == 0 {
        return Err(PencilError::InvalidJuniorRatio.into());
    }

    let ratio = (junior_amount as u128)
        .checked_mul(10000)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_div(total_amount as u128)
        .ok_or(PencilError::ArithmeticOverflow)? as u16;

    require!(ratio >= min_ratio, PencilError::InvalidJuniorRatio);

    Ok(())
}

/// 计算 Senior 价格
pub fn calculate_senior_price(
    total_deposits: u64,
    grow_token_supply: u64,
    behind_principal: u64,
    max_interest: u64,
) -> Result<u64> {
    if grow_token_supply == 0 {
        return Ok(0);
    }

    let tvl = total_deposits
        .checked_add(behind_principal)
        .ok_or(PencilError::ArithmeticOverflow)?;

    let tvl = if tvl > max_interest.checked_add(behind_principal).ok_or(PencilError::ArithmeticOverflow)? {
        max_interest.checked_add(behind_principal).ok_or(PencilError::ArithmeticOverflow)?
    } else {
        tvl
    };

    let price = (tvl as u128)
        .checked_mul(1_000_000)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_div(grow_token_supply as u128)
        .ok_or(PencilError::ArithmeticOverflow)? as u64;

    Ok(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_interest() {
        // 1000 tokens at 10% for 1 year
        let interest = calculate_interest(1000, 1000, 365).unwrap();
        assert_eq!(interest, 100);
    }

    #[test]
    fn test_calculate_fee() {
        // 1000 tokens with 1% fee
        let fee = calculate_fee(1000, 100).unwrap();
        assert_eq!(fee, 10);
    }

    #[test]
    fn test_calculate_current_period() {
        let funding_end_time = 1000;
        let repayment_period = 30; // 30 days
        let current_time = 1000 + (30 * 86400); // 30 days later

        let period = calculate_current_period(funding_end_time, repayment_period, current_time).unwrap();
        assert_eq!(period, 1);
    }

    #[test]
    fn test_validate_junior_ratio() {
        // 30% junior ratio, min 20%
        let result = validate_junior_ratio(3000, 10000, 2000);
        assert!(result.is_ok());

        // 10% junior ratio, min 20%
        let result = validate_junior_ratio(1000, 10000, 2000);
        assert!(result.is_err());
    }
}

