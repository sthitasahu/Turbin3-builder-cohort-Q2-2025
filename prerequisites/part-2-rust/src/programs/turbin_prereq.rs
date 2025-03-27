use solana_idlgen::idlgen;

idlgen!({
    "version": "0.1.0",
    "name": "turbin",
    "instructions": [
        {
            "name": "complete",
            "discriminator": [0, 77, 224, 147, 136, 25, 88, 76],
            "accounts": [
                {
                    "name": "signer",
                    "type": {
                        "kind": "account",
                        "fields": []
                    },
                    "writable": true,
                    "signer": true,
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "type": {
                        "kind": "account",
                        "fields": []
                    },
                    "writable": true,
                    "isMut": true,
                    "isSigner": false,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [112, 114, 101, 114, 101, 113]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "system_program",
                    "type": {
                        "kind": "program",
                        "fields": []
                    },
                    "address": "11111111111111111111111111111111",
                    "isMut": false,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        },
        {
            "name": "update",
            "discriminator": [219, 200, 88, 176, 158, 63, 253, 127],
            "accounts": [
                {
                    "name": "signer",
                    "type": {
                        "kind": "account",
                        "fields": []
                    },
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "type": {
                        "kind": "account",
                        "fields": []
                    },
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "systemProgram",
                    "type": {
                        "kind": "program",
                        "fields": []
                    },
                    "isMut": false,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        }
    ],
    "accounts": [
        {
            "name": "SolanaCohort5Account",
            "type": {
                "kind": "account",
                "fields": []
            },
            "discriminator": [167, 81, 85, 136, 32, 169, 137, 77]
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "InvalidGithubAccount",
            "msg": "Invalid Github account"
        }
    ],
    "types": [
        {
            "name": "SolanaCohort5Account",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "github",
                        "type": "bytes"
                    },
                    {
                        "name": "key",
                        "type": "pubkey"
                    }
                ]
            }
        }
    ],
    "metadata": {
        "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
    }
});