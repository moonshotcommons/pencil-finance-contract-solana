/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/pencil_solana.json`.
 */
export type PencilSolana = {
  "address": "RXo7Ai9ugeBp9giAKqera2pg1xMj49exA5SgWdMBMuM",
  "metadata": {
    "name": "pencilSolana",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "approveAssetPool",
      "discriminator": [
        240,
        141,
        145,
        219,
        82,
        141,
        53,
        129
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "creator"
              },
              {
                "kind": "arg",
                "path": "name"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "creator",
          "type": "pubkey"
        },
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "burnGrowToken",
      "discriminator": [
        174,
        79,
        45,
        27,
        45,
        121,
        44,
        244
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "growTokenMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  114,
                  111,
                  119,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "assetPool"
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "growTokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancelAssetPool",
      "discriminator": [
        20,
        79,
        53,
        120,
        31,
        135,
        11,
        210
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true
        },
        {
          "name": "assetMint"
        }
      ],
      "args": []
    },
    {
      "name": "claimJuniorInterest",
      "discriminator": [
        101,
        78,
        157,
        190,
        168,
        212,
        158,
        28
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "firstLossPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  114,
                  115,
                  116,
                  95,
                  108,
                  111,
                  115,
                  115,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "juniorInterestPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  105,
                  110,
                  116,
                  101,
                  114,
                  101,
                  115,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "nftMetadata",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "userNftAccount",
          "writable": true
        },
        {
          "name": "juniorNftMint",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "userAssetAccount",
          "writable": true
        },
        {
          "name": "assetPoolVault",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "nftId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "completeFunding",
      "discriminator": [
        142,
        83,
        61,
        200,
        110,
        57,
        199,
        183
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "seniorPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  110,
                  105,
                  111,
                  114,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "firstLossPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  114,
                  115,
                  116,
                  95,
                  108,
                  111,
                  115,
                  115,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "createAssetPool",
      "discriminator": [
        25,
        139,
        122,
        214,
        3,
        64,
        63,
        181
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetWhitelist",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  119,
                  104,
                  105,
                  116,
                  101,
                  108,
                  105,
                  115,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "payer"
              },
              {
                "kind": "arg",
                "path": "name"
              }
            ]
          }
        },
        {
          "name": "assetAddress",
          "docs": [
            "资产代币地址"
          ]
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "platformFee",
          "type": "u16"
        },
        {
          "name": "seniorEarlyBeforeExitFee",
          "type": "u16"
        },
        {
          "name": "seniorEarlyAfterExitFee",
          "type": "u16"
        },
        {
          "name": "juniorEarlyBeforeExitFee",
          "type": "u16"
        },
        {
          "name": "minJuniorRatio",
          "type": "u16"
        },
        {
          "name": "repaymentRate",
          "type": "u16"
        },
        {
          "name": "seniorFixedRate",
          "type": "u16"
        },
        {
          "name": "repaymentPeriod",
          "type": "u64"
        },
        {
          "name": "repaymentCount",
          "type": "u64"
        },
        {
          "name": "totalAmount",
          "type": "u64"
        },
        {
          "name": "minAmount",
          "type": "u64"
        },
        {
          "name": "fundingStartTime",
          "type": "i64"
        },
        {
          "name": "fundingEndTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "distributeJuniorNft",
      "discriminator": [
        0,
        254,
        209,
        124,
        76,
        72,
        138,
        50
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "subscription",
          "writable": true
        },
        {
          "name": "juniorNftMint",
          "docs": [
            "Junior NFT Mint PDA - 每个 NFT 有独立的 Mint"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "user"
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "juniorNftMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "nftMetadata",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "nftId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "distributeSeniorToken",
      "discriminator": [
        24,
        134,
        198,
        134,
        93,
        92,
        222,
        198
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "subscription",
          "writable": true
        },
        {
          "name": "growTokenMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  114,
                  111,
                  119,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "user"
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "growTokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "earlyExitSenior",
      "discriminator": [
        186,
        7,
        177,
        116,
        250,
        140,
        100,
        104
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "seniorPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  110,
                  105,
                  111,
                  114,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "firstLossPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  114,
                  115,
                  116,
                  95,
                  108,
                  111,
                  115,
                  115,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "growTokenMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  114,
                  111,
                  119,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "userGrowTokenAccount",
          "writable": true
        },
        {
          "name": "userAssetAccount",
          "writable": true
        },
        {
          "name": "assetPoolVault",
          "writable": true
        },
        {
          "name": "treasuryAta",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "finalizeTokenDistribution",
      "discriminator": [
        158,
        131,
        100,
        20,
        34,
        208,
        236,
        74
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool"
        }
      ],
      "args": [
        {
          "name": "seniorCount",
          "type": "u64"
        },
        {
          "name": "juniorCount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializeRelatedAccounts",
      "discriminator": [
        127,
        200,
        3,
        223,
        65,
        26,
        232,
        37
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "assetMint",
          "docs": [
            "资产代币 Mint"
          ]
        },
        {
          "name": "funding",
          "docs": [
            "Funding PDA 账户"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  117,
                  110,
                  100,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "seniorPool",
          "docs": [
            "SeniorPool PDA 账户"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  110,
                  105,
                  111,
                  114,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "firstLossPool",
          "docs": [
            "FirstLossPool PDA 账户"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  114,
                  115,
                  116,
                  95,
                  108,
                  111,
                  115,
                  115,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "juniorInterestPool",
          "docs": [
            "JuniorInterestPool PDA 账户"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  105,
                  110,
                  116,
                  101,
                  114,
                  101,
                  115,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "growTokenMint",
          "docs": [
            "GROW Token Mint PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  114,
                  111,
                  119,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "juniorNftMint",
          "docs": [
            "Junior NFT Mint PDA (用作基础 mint，实际 NFT 会有独立的 mint)"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "assetPoolVault",
          "docs": [
            "资产池 Token Vault ATA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "treasury",
          "docs": [
            "金库账户 (从 SystemConfig 读取)"
          ]
        },
        {
          "name": "treasuryAta",
          "docs": [
            "金库 ATA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "treasury"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializeSystemConfig",
      "discriminator": [
        43,
        153,
        196,
        116,
        233,
        36,
        208,
        246
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "treasury",
          "docs": [
            "金库账户"
          ],
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "platformFeeRate",
          "type": "u16"
        },
        {
          "name": "seniorEarlyBeforeExitFeeRate",
          "type": "u16"
        },
        {
          "name": "seniorEarlyAfterExitFeeRate",
          "type": "u16"
        },
        {
          "name": "juniorEarlyBeforeExitFeeRate",
          "type": "u16"
        },
        {
          "name": "defaultMinJuniorRatio",
          "type": "u16"
        }
      ]
    },
    {
      "name": "mintGrowToken",
      "discriminator": [
        17,
        195,
        170,
        88,
        160,
        13,
        221,
        2
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "growTokenMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  114,
                  111,
                  119,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "recipient"
        },
        {
          "name": "recipientTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "recipient"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "growTokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "mintJuniorNft",
      "discriminator": [
        41,
        235,
        58,
        29,
        24,
        231,
        244,
        161
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "juniorNftMint",
          "docs": [
            "Junior NFT Mint PDA - 每个 NFT 有独立的 Mint"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "recipient"
        },
        {
          "name": "recipientTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "recipient"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "juniorNftMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "nftMetadata",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "nftId",
          "type": "u64"
        },
        {
          "name": "principal",
          "type": "u64"
        }
      ]
    },
    {
      "name": "pauseSystem",
      "discriminator": [
        61,
        33,
        120,
        81,
        219,
        224,
        186,
        28
      ],
      "accounts": [
        {
          "name": "superAdmin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "processRefund",
      "discriminator": [
        207,
        200,
        62,
        144,
        204,
        189,
        222,
        226
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "subscription",
          "writable": true
        },
        {
          "name": "poolVault",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "refundSubscription",
      "discriminator": [
        31,
        146,
        27,
        46,
        118,
        79,
        66,
        191
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "subscription",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "subscriptionType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "repay",
      "discriminator": [
        234,
        103,
        67,
        82,
        208,
        234,
        219,
        166
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetWhitelist",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  119,
                  104,
                  105,
                  116,
                  101,
                  108,
                  105,
                  115,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "seniorPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  110,
                  105,
                  111,
                  114,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "firstLossPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  114,
                  115,
                  116,
                  95,
                  108,
                  111,
                  115,
                  115,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "juniorInterestPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  105,
                  110,
                  116,
                  101,
                  114,
                  101,
                  115,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "payerTokenAccount",
          "writable": true
        },
        {
          "name": "assetPoolVault",
          "writable": true
        },
        {
          "name": "treasuryAta",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "repaymentRecord",
          "docs": [
            "Repayment record - one per period"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  112,
                  97,
                  121,
                  109,
                  101,
                  110,
                  116,
                  95,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "period"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "period",
          "type": "u64"
        }
      ]
    },
    {
      "name": "setAssetSupported",
      "discriminator": [
        81,
        182,
        114,
        225,
        217,
        107,
        120,
        93
      ],
      "accounts": [
        {
          "name": "operationAdmin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetWhitelist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  119,
                  104,
                  105,
                  116,
                  101,
                  108,
                  105,
                  115,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "asset",
          "type": "pubkey"
        },
        {
          "name": "supported",
          "type": "bool"
        }
      ]
    },
    {
      "name": "setTreasury",
      "discriminator": [
        57,
        97,
        196,
        95,
        195,
        206,
        106,
        136
      ],
      "accounts": [
        {
          "name": "systemAdmin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "treasury",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "subscribeJunior",
      "discriminator": [
        197,
        150,
        82,
        182,
        216,
        198,
        135,
        137
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetWhitelist",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  119,
                  104,
                  105,
                  116,
                  101,
                  108,
                  105,
                  115,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "poolTokenAccount",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "subscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  117,
                  98,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "subscribeSenior",
      "discriminator": [
        47,
        102,
        28,
        1,
        246,
        75,
        51,
        207
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetWhitelist",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  119,
                  104,
                  105,
                  116,
                  101,
                  108,
                  105,
                  115,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "poolTokenAccount",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "subscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  117,
                  98,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  110,
                  105,
                  111,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "unpauseSystem",
      "discriminator": [
        26,
        131,
        200,
        89,
        124,
        244,
        140,
        236
      ],
      "accounts": [
        {
          "name": "superAdmin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "updateAdmin",
      "discriminator": [
        161,
        176,
        40,
        213,
        60,
        184,
        179,
        228
      ],
      "accounts": [
        {
          "name": "superAdmin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "role",
          "type": {
            "defined": {
              "name": "adminRole"
            }
          }
        },
        {
          "name": "newAdmin",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "updateFeeRate",
      "discriminator": [
        195,
        241,
        226,
        216,
        102,
        1,
        5,
        122
      ],
      "accounts": [
        {
          "name": "systemAdmin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "feeType",
          "type": {
            "defined": {
              "name": "feeType"
            }
          }
        },
        {
          "name": "newRate",
          "type": "u16"
        }
      ]
    },
    {
      "name": "withdrawJuniorSubscription",
      "discriminator": [
        71,
        77,
        54,
        90,
        7,
        25,
        31,
        130
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "subscription",
          "writable": true
        },
        {
          "name": "poolTokenAccount",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "treasuryAta",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "treasury"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawPrincipal",
      "discriminator": [
        6,
        59,
        175,
        16,
        210,
        146,
        119,
        63
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  115,
                  115,
                  101,
                  116,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "asset_pool.creator",
                "account": "assetPool"
              },
              {
                "kind": "account",
                "path": "asset_pool.name",
                "account": "assetPool"
              }
            ]
          }
        },
        {
          "name": "firstLossPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  114,
                  115,
                  116,
                  95,
                  108,
                  111,
                  115,
                  115,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              }
            ]
          }
        },
        {
          "name": "nftMetadata",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  101,
                  116,
                  97,
                  100,
                  97,
                  116,
                  97
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "userNftAccount",
          "writable": true
        },
        {
          "name": "juniorNftMint",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  106,
                  117,
                  110,
                  105,
                  111,
                  114,
                  95,
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "assetPool"
              },
              {
                "kind": "arg",
                "path": "nftId"
              }
            ]
          }
        },
        {
          "name": "userAssetAccount",
          "writable": true
        },
        {
          "name": "assetPoolVault",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "nftId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawSeniorSubscription",
      "discriminator": [
        116,
        153,
        205,
        101,
        184,
        234,
        66,
        105
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "assetPool",
          "writable": true
        },
        {
          "name": "subscription",
          "writable": true
        },
        {
          "name": "poolTokenAccount",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "treasuryAta",
          "writable": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "treasury"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "assetPool",
      "discriminator": [
        81,
        48,
        2,
        215,
        147,
        255,
        152,
        112
      ]
    },
    {
      "name": "assetWhitelist",
      "discriminator": [
        105,
        100,
        133,
        187,
        165,
        205,
        77,
        206
      ]
    },
    {
      "name": "firstLossPool",
      "discriminator": [
        232,
        63,
        126,
        121,
        212,
        77,
        194,
        181
      ]
    },
    {
      "name": "funding",
      "discriminator": [
        50,
        175,
        214,
        196,
        200,
        110,
        145,
        161
      ]
    },
    {
      "name": "juniorInterestPool",
      "discriminator": [
        239,
        45,
        95,
        52,
        21,
        175,
        87,
        193
      ]
    },
    {
      "name": "juniorNftMetadata",
      "discriminator": [
        69,
        9,
        208,
        120,
        105,
        109,
        113,
        125
      ]
    },
    {
      "name": "repaymentRecord",
      "discriminator": [
        194,
        187,
        97,
        21,
        42,
        0,
        189,
        162
      ]
    },
    {
      "name": "seniorPool",
      "discriminator": [
        60,
        12,
        54,
        67,
        1,
        156,
        90,
        115
      ]
    },
    {
      "name": "subscription",
      "discriminator": [
        64,
        7,
        26,
        135,
        102,
        132,
        98,
        33
      ]
    },
    {
      "name": "systemConfig",
      "discriminator": [
        218,
        150,
        16,
        126,
        102,
        185,
        75,
        1
      ]
    }
  ],
  "events": [
    {
      "name": "adminUpdated",
      "discriminator": [
        69,
        82,
        49,
        171,
        43,
        3,
        80,
        161
      ]
    },
    {
      "name": "assetSupportUpdated",
      "discriminator": [
        115,
        248,
        105,
        199,
        219,
        94,
        75,
        142
      ]
    },
    {
      "name": "earlyExitProcessed",
      "discriminator": [
        26,
        228,
        175,
        113,
        92,
        25,
        155,
        234
      ]
    },
    {
      "name": "feeRateUpdated",
      "discriminator": [
        90,
        28,
        42,
        224,
        39,
        78,
        81,
        27
      ]
    },
    {
      "name": "interestClaimed",
      "discriminator": [
        207,
        228,
        188,
        90,
        131,
        122,
        6,
        125
      ]
    },
    {
      "name": "principalWithdrawn",
      "discriminator": [
        156,
        68,
        7,
        220,
        207,
        134,
        122,
        12
      ]
    },
    {
      "name": "refundProcessed",
      "discriminator": [
        203,
        88,
        236,
        233,
        192,
        178,
        57,
        161
      ]
    },
    {
      "name": "relatedAccountsInitialized",
      "discriminator": [
        104,
        194,
        128,
        60,
        48,
        141,
        121,
        191
      ]
    },
    {
      "name": "repaymentDistributed",
      "discriminator": [
        204,
        36,
        66,
        245,
        248,
        101,
        151,
        187
      ]
    },
    {
      "name": "systemPaused",
      "discriminator": [
        199,
        92,
        193,
        5,
        14,
        241,
        182,
        17
      ]
    },
    {
      "name": "systemUnpaused",
      "discriminator": [
        29,
        28,
        211,
        10,
        79,
        179,
        175,
        13
      ]
    },
    {
      "name": "tokensDistributed",
      "discriminator": [
        117,
        252,
        224,
        3,
        212,
        156,
        207,
        43
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidSystemConfig",
      "msg": "Invalid system configuration"
    },
    {
      "code": 6001,
      "name": "invalidAssetPool",
      "msg": "Invalid asset pool"
    },
    {
      "code": 6002,
      "name": "invalidFundingParams",
      "msg": "Invalid funding parameters"
    },
    {
      "code": 6003,
      "name": "fundingNotStarted",
      "msg": "Funding not started"
    },
    {
      "code": 6004,
      "name": "fundingEnded",
      "msg": "Funding already ended"
    },
    {
      "code": 6005,
      "name": "fundingNotCompleted",
      "msg": "Funding not completed"
    },
    {
      "code": 6006,
      "name": "invalidSubscriptionAmount",
      "msg": "Invalid subscription amount"
    },
    {
      "code": 6007,
      "name": "insufficientBalance",
      "msg": "Insufficient balance"
    },
    {
      "code": 6008,
      "name": "insufficientSeniorAmount",
      "msg": "Insufficient senior amount"
    },
    {
      "code": 6009,
      "name": "insufficientJuniorAmount",
      "msg": "Insufficient junior amount"
    },
    {
      "code": 6010,
      "name": "invalidJuniorRatio",
      "msg": "Invalid junior ratio"
    },
    {
      "code": 6011,
      "name": "invalidRepaymentAmount",
      "msg": "Invalid repayment amount"
    },
    {
      "code": 6012,
      "name": "invalidRepaymentPeriod",
      "msg": "Invalid repayment period"
    },
    {
      "code": 6013,
      "name": "invalidRepaymentCount",
      "msg": "Invalid repayment count"
    },
    {
      "code": 6014,
      "name": "repaymentNotDue",
      "msg": "Repayment not due"
    },
    {
      "code": 6015,
      "name": "repaymentAlreadyCompleted",
      "msg": "Repayment already completed"
    },
    {
      "code": 6016,
      "name": "invalidEarlyExitFee",
      "msg": "Invalid early exit fee"
    },
    {
      "code": 6017,
      "name": "invalidPlatformFee",
      "msg": "Invalid platform fee"
    },
    {
      "code": 6018,
      "name": "invalidSeniorFixedRate",
      "msg": "Invalid senior fixed rate"
    },
    {
      "code": 6019,
      "name": "invalidRepaymentRate",
      "msg": "Invalid repayment rate"
    },
    {
      "code": 6020,
      "name": "invalidMinJuniorRatio",
      "msg": "Invalid min junior ratio"
    },
    {
      "code": 6021,
      "name": "unauthorized",
      "msg": "unauthorized"
    },
    {
      "code": 6022,
      "name": "invalidAccount",
      "msg": "Invalid account"
    },
    {
      "code": 6023,
      "name": "invalidTokenAccount",
      "msg": "Invalid token account"
    },
    {
      "code": 6024,
      "name": "invalidMint",
      "msg": "Invalid mint"
    },
    {
      "code": 6025,
      "name": "transferFailed",
      "msg": "Transfer failed"
    },
    {
      "code": 6026,
      "name": "mintFailed",
      "msg": "Mint failed"
    },
    {
      "code": 6027,
      "name": "burnFailed",
      "msg": "Burn failed"
    },
    {
      "code": 6028,
      "name": "invalidNft",
      "msg": "Invalid NFT"
    },
    {
      "code": 6029,
      "name": "nftAlreadyClaimed",
      "msg": "NFT already claimed"
    },
    {
      "code": 6030,
      "name": "invalidSubscriptionStatus",
      "msg": "Invalid subscription status"
    },
    {
      "code": 6031,
      "name": "invalidAssetPoolStatus",
      "msg": "Invalid asset pool status"
    },
    {
      "code": 6032,
      "name": "assetPoolAlreadyApproved",
      "msg": "Asset pool already approved"
    },
    {
      "code": 6033,
      "name": "assetPoolNotApproved",
      "msg": "Asset pool not approved"
    },
    {
      "code": 6034,
      "name": "fundingTargetNotMet",
      "msg": "Funding target not met"
    },
    {
      "code": 6035,
      "name": "fundingMinimumNotMet",
      "msg": "Funding minimum not met"
    },
    {
      "code": 6036,
      "name": "invalidTimeParameters",
      "msg": "Invalid time parameters"
    },
    {
      "code": 6037,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6038,
      "name": "invalidPoolAddress",
      "msg": "Invalid pool address"
    },
    {
      "code": 6039,
      "name": "invalidTreasuryAddress",
      "msg": "Invalid treasury address"
    },
    {
      "code": 6040,
      "name": "insufficientPoolFunds",
      "msg": "Insufficient funds in pool"
    },
    {
      "code": 6041,
      "name": "invalidInterestCalculation",
      "msg": "Invalid interest calculation"
    },
    {
      "code": 6042,
      "name": "invalidPrincipalCalculation",
      "msg": "Invalid principal calculation"
    },
    {
      "code": 6043,
      "name": "subscriptionNotFound",
      "msg": "Subscription not found"
    },
    {
      "code": 6044,
      "name": "repaymentRecordNotFound",
      "msg": "Repayment record not found"
    },
    {
      "code": 6045,
      "name": "invalidStringLength",
      "msg": "Invalid string length"
    },
    {
      "code": 6046,
      "name": "duplicateSubscription",
      "msg": "Duplicate subscription"
    },
    {
      "code": 6047,
      "name": "invalidRefundAmount",
      "msg": "Invalid refund amount"
    },
    {
      "code": 6048,
      "name": "refundAlreadyProcessed",
      "msg": "Refund already processed"
    },
    {
      "code": 6049,
      "name": "invalidClaimAmount",
      "msg": "Invalid claim amount"
    },
    {
      "code": 6050,
      "name": "noInterestToClaim",
      "msg": "No interest to claim"
    },
    {
      "code": 6051,
      "name": "noPrincipalToWithdraw",
      "msg": "No principal to withdraw"
    },
    {
      "code": 6052,
      "name": "invalidEarlyExitAmount",
      "msg": "Invalid early exit amount"
    },
    {
      "code": 6053,
      "name": "earlyExitNotAllowed",
      "msg": "Early exit not allowed"
    },
    {
      "code": 6054,
      "name": "invalidTokenDecimals",
      "msg": "Invalid token decimals"
    },
    {
      "code": 6055,
      "name": "precisionLoss",
      "msg": "Precision loss in calculation"
    },
    {
      "code": 6056,
      "name": "systemPaused",
      "msg": "System is paused"
    },
    {
      "code": 6057,
      "name": "invalidAdminRole",
      "msg": "Invalid admin role"
    },
    {
      "code": 6058,
      "name": "invalidFeeType",
      "msg": "Invalid fee type"
    },
    {
      "code": 6059,
      "name": "assetNotSupported",
      "msg": "Asset not in whitelist"
    },
    {
      "code": 6060,
      "name": "relatedAccountsAlreadyInitialized",
      "msg": "Related accounts already initialized"
    },
    {
      "code": 6061,
      "name": "relatedAccountsNotInitialized",
      "msg": "Related accounts not initialized"
    },
    {
      "code": 6062,
      "name": "insufficientVaultBalance",
      "msg": "Insufficient vault balance"
    },
    {
      "code": 6063,
      "name": "invalidEarlyExitTiming",
      "msg": "Invalid early exit timing"
    },
    {
      "code": 6064,
      "name": "nftNotOwnedByUser",
      "msg": "NFT not owned by user"
    },
    {
      "code": 6065,
      "name": "principalAlreadyWithdrawn",
      "msg": "Principal already withdrawn"
    },
    {
      "code": 6066,
      "name": "poolNotEnded",
      "msg": "Pool not ended"
    },
    {
      "code": 6067,
      "name": "invalidPeriodCalculation",
      "msg": "Invalid period calculation"
    }
  ],
  "types": [
    {
      "name": "adminRole",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "superAdmin"
          },
          {
            "name": "systemAdmin"
          },
          {
            "name": "treasuryAdmin"
          },
          {
            "name": "operationAdmin"
          }
        ]
      }
    },
    {
      "name": "adminUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "role",
            "type": "u8"
          },
          {
            "name": "oldAdmin",
            "type": "pubkey"
          },
          {
            "name": "newAdmin",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "assetPool",
      "docs": [
        "资产池账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "docs": [
              "资产池名称"
            ],
            "type": "string"
          },
          {
            "name": "status",
            "docs": [
              "资产池状态"
            ],
            "type": "u8"
          },
          {
            "name": "assetAddress",
            "docs": [
              "资产代币地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "systemConfig",
            "docs": [
              "系统配置地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "funding",
            "docs": [
              "募资合约地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "seniorPool",
            "docs": [
              "优先池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "firstLossPool",
            "docs": [
              "首损池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "juniorInterestPool",
            "docs": [
              "利息池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "growToken",
            "docs": [
              "GROW Token 地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "juniorNft",
            "docs": [
              "Junior NFT 地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "docs": [
              "金库地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "platformFee",
            "docs": [
              "平台手续费 (基点)"
            ],
            "type": "u16"
          },
          {
            "name": "seniorEarlyBeforeExitFee",
            "docs": [
              "优先份额募资结束前提前退出手续费"
            ],
            "type": "u16"
          },
          {
            "name": "seniorEarlyAfterExitFee",
            "docs": [
              "优先份额募资结束后提前退出手续费"
            ],
            "type": "u16"
          },
          {
            "name": "juniorEarlyBeforeExitFee",
            "docs": [
              "次级份额募资结束前提前退出手续费"
            ],
            "type": "u16"
          },
          {
            "name": "minJuniorRatio",
            "docs": [
              "最低 Junior 占比"
            ],
            "type": "u16"
          },
          {
            "name": "repaymentRate",
            "docs": [
              "还款利率 (基点)"
            ],
            "type": "u16"
          },
          {
            "name": "seniorFixedRate",
            "docs": [
              "优先份额固定利率 (基点)"
            ],
            "type": "u16"
          },
          {
            "name": "repaymentPeriod",
            "docs": [
              "还款周期 (天)"
            ],
            "type": "u64"
          },
          {
            "name": "repaymentCount",
            "docs": [
              "还款期数"
            ],
            "type": "u64"
          },
          {
            "name": "totalAmount",
            "docs": [
              "预期募资总金额"
            ],
            "type": "u64"
          },
          {
            "name": "minAmount",
            "docs": [
              "最少募资目标"
            ],
            "type": "u64"
          },
          {
            "name": "fundingStartTime",
            "docs": [
              "募资开始时间"
            ],
            "type": "i64"
          },
          {
            "name": "fundingEndTime",
            "docs": [
              "募资结束时间"
            ],
            "type": "i64"
          },
          {
            "name": "seniorAmount",
            "docs": [
              "优先份额金额"
            ],
            "type": "u64"
          },
          {
            "name": "juniorAmount",
            "docs": [
              "次级份额金额"
            ],
            "type": "u64"
          },
          {
            "name": "repaidAmount",
            "docs": [
              "已还款金额"
            ],
            "type": "u64"
          },
          {
            "name": "creator",
            "docs": [
              "创建者"
            ],
            "type": "pubkey"
          },
          {
            "name": "createdAt",
            "docs": [
              "创建时间"
            ],
            "type": "i64"
          },
          {
            "name": "relatedAccountsInitialized",
            "docs": [
              "相关账户是否已初始化"
            ],
            "type": "bool"
          },
          {
            "name": "assetPoolVault",
            "docs": [
              "资产池 Token Vault ATA"
            ],
            "type": "pubkey"
          },
          {
            "name": "treasuryAta",
            "docs": [
              "金库 ATA"
            ],
            "type": "pubkey"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                63
              ]
            }
          }
        ]
      }
    },
    {
      "name": "assetSupportUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "asset",
            "type": "pubkey"
          },
          {
            "name": "supported",
            "type": "bool"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "assetWhitelist",
      "docs": [
        "资产白名单账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "systemConfig",
            "docs": [
              "系统配置地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "assets",
            "docs": [
              "支持的资产列表"
            ],
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "earlyExitProcessed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "netAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "feeRateUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeType",
            "type": "u8"
          },
          {
            "name": "oldRate",
            "type": "u16"
          },
          {
            "name": "newRate",
            "type": "u16"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "feeType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "platformFee"
          },
          {
            "name": "seniorEarlyBeforeExitFee"
          },
          {
            "name": "seniorEarlyAfterExitFee"
          },
          {
            "name": "juniorEarlyBeforeExitFee"
          }
        ]
      }
    },
    {
      "name": "firstLossPool",
      "docs": [
        "首损池账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "juniorNft",
            "docs": [
              "Junior NFT 地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "totalDeposits",
            "docs": [
              "总存款金额"
            ],
            "type": "u64"
          },
          {
            "name": "repaidAmount",
            "docs": [
              "已还款金额"
            ],
            "type": "u64"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "funding",
      "docs": [
        "募资账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "assetAddress",
            "docs": [
              "资产代币地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "seniorTotal",
            "docs": [
              "优先份额总认购金额"
            ],
            "type": "u64"
          },
          {
            "name": "juniorTotal",
            "docs": [
              "次级份额总认购金额"
            ],
            "type": "u64"
          },
          {
            "name": "status",
            "docs": [
              "募资状态"
            ],
            "type": "u8"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "interestClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "nftId",
            "type": "u64"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "juniorInterestPool",
      "docs": [
        "利息池账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "juniorNft",
            "docs": [
              "Junior NFT 地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "totalInterest",
            "docs": [
              "总利息金额"
            ],
            "type": "u64"
          },
          {
            "name": "distributedInterest",
            "docs": [
              "已分配利息"
            ],
            "type": "u64"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "juniorNftMetadata",
      "docs": [
        "Junior NFT 元数据"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "nftId",
            "docs": [
              "NFT ID"
            ],
            "type": "u64"
          },
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "owner",
            "docs": [
              "所有者地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "principal",
            "docs": [
              "本金金额"
            ],
            "type": "u64"
          },
          {
            "name": "claimedInterest",
            "docs": [
              "已领取利息"
            ],
            "type": "u64"
          },
          {
            "name": "principalWithdrawn",
            "docs": [
              "是否已提取本金"
            ],
            "type": "bool"
          },
          {
            "name": "createdAt",
            "docs": [
              "创建时间"
            ],
            "type": "i64"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "principalWithdrawn",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "nftId",
            "type": "u64"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "refundProcessed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "subscriptionType",
            "type": "u8"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "relatedAccountsInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "funding",
            "type": "pubkey"
          },
          {
            "name": "seniorPool",
            "type": "pubkey"
          },
          {
            "name": "firstLossPool",
            "type": "pubkey"
          },
          {
            "name": "juniorInterestPool",
            "type": "pubkey"
          },
          {
            "name": "growToken",
            "type": "pubkey"
          },
          {
            "name": "assetPoolVault",
            "type": "pubkey"
          },
          {
            "name": "treasuryAta",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "repaymentDistributed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "period",
            "type": "u64"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "platformFee",
            "type": "u64"
          },
          {
            "name": "seniorAmount",
            "type": "u64"
          },
          {
            "name": "juniorInterest",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "repaymentRecord",
      "docs": [
        "还款记录"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "period",
            "docs": [
              "还款期数"
            ],
            "type": "u64"
          },
          {
            "name": "amount",
            "docs": [
              "还款金额"
            ],
            "type": "u64"
          },
          {
            "name": "repaidAt",
            "docs": [
              "还款时间"
            ],
            "type": "i64"
          },
          {
            "name": "status",
            "docs": [
              "还款状态"
            ],
            "type": "u8"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "seniorPool",
      "docs": [
        "优先池账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "growToken",
            "docs": [
              "GROW Token 地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "totalDeposits",
            "docs": [
              "总存款金额"
            ],
            "type": "u64"
          },
          {
            "name": "repaidAmount",
            "docs": [
              "已还款金额"
            ],
            "type": "u64"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "subscription",
      "docs": [
        "用户订阅记录"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "docs": [
              "资产池地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "user",
            "docs": [
              "用户地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "subscriptionType",
            "docs": [
              "订阅类型 (0: senior, 1: junior)"
            ],
            "type": "u8"
          },
          {
            "name": "amount",
            "docs": [
              "订阅金额"
            ],
            "type": "u64"
          },
          {
            "name": "status",
            "docs": [
              "订阅状态"
            ],
            "type": "u8"
          },
          {
            "name": "subscribedAt",
            "docs": [
              "订阅时间"
            ],
            "type": "i64"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "systemConfig",
      "docs": [
        "系统配置账户"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "superAdmin",
            "docs": [
              "超级管理员"
            ],
            "type": "pubkey"
          },
          {
            "name": "systemAdmin",
            "docs": [
              "系统管理员"
            ],
            "type": "pubkey"
          },
          {
            "name": "treasuryAdmin",
            "docs": [
              "金库管理员"
            ],
            "type": "pubkey"
          },
          {
            "name": "operationAdmin",
            "docs": [
              "运营管理员"
            ],
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "docs": [
              "金库地址"
            ],
            "type": "pubkey"
          },
          {
            "name": "platformFeeRate",
            "docs": [
              "平台手续费率 (基点)"
            ],
            "type": "u16"
          },
          {
            "name": "seniorEarlyBeforeExitFeeRate",
            "docs": [
              "优先份额募资结束前提前退出手续费率"
            ],
            "type": "u16"
          },
          {
            "name": "seniorEarlyAfterExitFeeRate",
            "docs": [
              "优先份额募资结束后提前退出手续费率"
            ],
            "type": "u16"
          },
          {
            "name": "juniorEarlyBeforeExitFeeRate",
            "docs": [
              "次级份额募资结束前提前退出手续费率"
            ],
            "type": "u16"
          },
          {
            "name": "defaultMinJuniorRatio",
            "docs": [
              "默认最低 Junior 占比"
            ],
            "type": "u16"
          },
          {
            "name": "initialized",
            "docs": [
              "是否已初始化"
            ],
            "type": "bool"
          },
          {
            "name": "paused",
            "docs": [
              "系统暂停状态"
            ],
            "type": "bool"
          },
          {
            "name": "reserved",
            "docs": [
              "预留空间"
            ],
            "type": {
              "array": [
                "u8",
                127
              ]
            }
          }
        ]
      }
    },
    {
      "name": "systemPaused",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "systemUnpaused",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tokensDistributed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "assetPool",
            "type": "pubkey"
          },
          {
            "name": "seniorAmount",
            "type": "u64"
          },
          {
            "name": "juniorCount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
