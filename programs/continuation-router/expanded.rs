#![feature(prelude_import)]
//! Atomically routes a swap between multiple pools.
//!
//! To use this, create a transaction consisting of the following instructions:
//! 1. A [Begin] instruction
//! 2. Action instructions
//! 3. An [End] instruction
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use continuation_router_syn::router_action;
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};
use anchor_spl::token::{Token, TokenAccount};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use vipers::prelude::*;
pub mod action {
    use anchor_lang::prelude::*;
    use anchor_spl::token::TokenAccount;
    use crate::processor::ActionContext;
    pub mod stable_swap {
        //! Step implementations for StableSwap.
        use std::ops::Deref;
        use crate::action::ProcessAction;
        use crate::*;
        impl<'info> Deref for SSDepositA<'info> {
            type Target = SSDeposit<'info>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<'info> Deref for SSDepositB<'info> {
            type Target = SSDeposit<'info>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl<'info> ProcessAction<'info> for SSDepositA<'info> {
            /// Runs the deposit component instruction.
            fn process(
                ctx: &ActionContext<'_, '_, '_, 'info, Self>,
                amount_in: u64,
                minimum_amount_out: u64,
            ) -> Result<()> {
                let deposit = ctx.action;
                let cpi_accounts = stable_swap_anchor::Deposit {
                    user: {
                        stable_swap_anchor::SwapUserContext {
                            token_program: ctx.token_program.to_account_info(),
                            user_authority: ctx.owner.clone(),
                            swap: deposit.swap.swap.clone(),
                            swap_authority: deposit.swap.swap_authority.clone(),
                        }
                    },
                    input_a: (&deposit.input_a).into(),
                    input_b: (&deposit.input_b).into(),
                    output_lp: deposit.output_lp.to_account_info(),
                    pool_mint: deposit.pool_mint.clone(),
                };
                let cpi_ctx = CpiContext::new(ctx.swap_program.clone(), cpi_accounts);
                stable_swap_anchor::deposit(cpi_ctx, amount_in, 0, minimum_amount_out)
            }
            fn input_account(&self) -> &Account<'info, TokenAccount> {
                &self.input_a.user
            }
            fn output_account(&self) -> &Account<'info, TokenAccount> {
                &self.output_lp
            }
        }
        impl<'info> ProcessAction<'info> for SSDepositB<'info> {
            /// Runs the deposit component instruction.
            fn process(
                ctx: &ActionContext<'_, '_, '_, 'info, Self>,
                amount_in: u64,
                minimum_amount_out: u64,
            ) -> Result<()> {
                let deposit = ctx.action;
                let cpi_accounts = stable_swap_anchor::Deposit {
                    user: {
                        stable_swap_anchor::SwapUserContext {
                            token_program: ctx.token_program.to_account_info(),
                            user_authority: ctx.owner.clone(),
                            swap: deposit.swap.swap.clone(),
                            swap_authority: deposit.swap.swap_authority.clone(),
                        }
                    },
                    input_a: (&deposit.input_a).into(),
                    input_b: (&deposit.input_b).into(),
                    output_lp: deposit.output_lp.to_account_info(),
                    pool_mint: deposit.pool_mint.clone(),
                };
                let cpi_ctx = CpiContext::new(ctx.swap_program.clone(), cpi_accounts);
                stable_swap_anchor::deposit(cpi_ctx, 0, amount_in, minimum_amount_out)
            }
            fn input_account(&self) -> &Account<'info, TokenAccount> {
                &self.input_b.user
            }
            fn output_account(&self) -> &Account<'info, TokenAccount> {
                &self.output_lp
            }
        }
        impl<'info> ProcessAction<'info> for SSWithdrawOne<'info> {
            /// Runs the deposit component instruction.
            fn process(
                ctx: &ActionContext<'_, '_, '_, 'info, Self>,
                amount_in: u64,
                minimum_amount_out: u64,
            ) -> Result<()> {
                let action = ctx.action;
                let cpi_accounts = stable_swap_anchor::WithdrawOne {
                    user: {
                        stable_swap_anchor::SwapUserContext {
                            token_program: ctx.token_program.to_account_info(),
                            user_authority: ctx.owner.clone(),
                            swap: action.swap.swap.clone(),
                            swap_authority: action.swap.swap_authority.clone(),
                        }
                    },
                    pool_mint: action.pool_mint.clone(),
                    input_lp: action.input_lp.to_account_info(),
                    quote_reserves: action.quote_reserves.clone(),
                    output: (&action.output).into(),
                };
                let cpi_ctx = CpiContext::new(ctx.swap_program.clone(), cpi_accounts);
                stable_swap_anchor::withdraw_one(cpi_ctx, amount_in, minimum_amount_out)
            }
            fn input_account(&self) -> &Account<'info, TokenAccount> {
                &self.input_lp
            }
            fn output_account(&self) -> &Account<'info, TokenAccount> {
                &self.output.user_token.user
            }
        }
        impl<'info> ProcessAction<'info> for SSSwap<'info> {
            /// Runs the deposit component instruction.
            fn process(
                ctx: &ActionContext<'_, '_, '_, 'info, Self>,
                amount_in: u64,
                minimum_amount_out: u64,
            ) -> Result<()> {
                let action = ctx.action;
                let cpi_accounts = stable_swap_anchor::Swap {
                    user: {
                        stable_swap_anchor::SwapUserContext {
                            token_program: ctx.token_program.to_account_info(),
                            user_authority: ctx.owner.clone(),
                            swap: action.swap.swap.clone(),
                            swap_authority: action.swap.swap_authority.clone(),
                        }
                    },
                    input: (&action.input).into(),
                    output: (&action.output).into(),
                };
                let cpi_ctx = CpiContext::new(ctx.swap_program.clone(), cpi_accounts);
                stable_swap_anchor::swap(cpi_ctx, amount_in, minimum_amount_out)
            }
            fn input_account(&self) -> &Account<'info, TokenAccount> {
                &self.input.user
            }
            fn output_account(&self) -> &Account<'info, TokenAccount> {
                &self.output.user_token.user
            }
        }
        impl<'info> From<&SwapToken<'info>> for stable_swap_anchor::SwapToken<'info> {
            fn from(accounts: &SwapToken<'info>) -> stable_swap_anchor::SwapToken<'info> {
                stable_swap_anchor::SwapToken {
                    user: accounts.user.to_account_info(),
                    reserve: accounts.reserve.clone(),
                }
            }
        }
        impl<'info> From<&SwapOutput<'info>> for stable_swap_anchor::SwapOutput<'info> {
            fn from(accounts: &SwapOutput<'info>) -> stable_swap_anchor::SwapOutput<'info> {
                stable_swap_anchor::SwapOutput {
                    user_token: (&accounts.user_token).into(),
                    fees: accounts.fees.clone(),
                }
            }
        }
    }
    pub trait ProcessAction<'info>: Sized {
        /// Processes the action.
        fn process(
            ctx: &ActionContext<'_, '_, '_, 'info, Self>,
            amount_in: u64,
            minimum_amount_out: u64,
        ) -> Result<()>;
        fn input_account(&self) -> &Account<'info, TokenAccount>;
        fn output_account(&self) -> &Account<'info, TokenAccount>;
    }
}
pub mod processor {
    use anchor_lang::prelude::*;
    use anchor_spl::token::{Token, TokenAccount};
    use vipers::{assert_keys_eq, invariant};
    use crate::{Action, Continuation, SwapActionEvent, TokenAmount};
    pub trait ActionInputOutput<'info>: Action {
        fn input_account(&self) -> &Account<'info, TokenAccount>;
        fn output_account(&self) -> &Account<'info, TokenAccount>;
    }
    pub struct ActionContext<'a, 'b, 'c, 'info, T> {
        /// Currently executing program id.
        pub program_id: &'a Pubkey,
        /// Deserialized accounts.
        pub action: &'b T,
        /// Remaining accounts given but not deserialized or validated.
        /// Be very careful when using this directly.
        pub remaining_accounts: &'c [AccountInfo<'info>],
        /// The spl_token program.
        pub token_program: Program<'info, Token>,
        /// The relevant swap program.
        /// CHECK: Checked by executor
        pub swap_program: AccountInfo<'info>,
        /// The owner of all involved token accounts.
        /// CHECK: Arbitrary
        pub owner: AccountInfo<'info>,
    }
    /// Processes a context.
    pub trait Processor<'info>: ActionInputOutput<'info> {
        fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()>;
        fn process(&self, continuation: &mut Account<'info, Continuation>) -> Result<()> {
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Router action: "],
                    &[::core::fmt::ArgumentV1::new_debug(&Self::TYPE)],
                ));
                res
            });
            let continuation = continuation;
            {
                if !(continuation.steps_left > 0) {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&crate::ErrorCode::NoMoreSteps),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::NoMoreSteps,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("continuation.steps_left > 0");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &37u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::NoMoreSteps.name(),
                            error_code_number: crate::ErrorCode::NoMoreSteps.into(),
                            error_msg: crate::ErrorCode::NoMoreSteps.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 37u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            let input_account = self.input_account();
            {
                let __key_a = &input_account.key();
                let __key_b = &continuation.input;
                let __account_a = ::vipers::AsKeyRef::as_key_ref(__key_a);
                let __account_b = ::vipers::AsKeyRef::as_key_ref(__key_b);
                if __account_a != __account_b {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &crate::ErrorCode::PathInputOutputMismatch,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::PathInputOutputMismatch,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("input_account.key() != continuation.input");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Left: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_a)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Right: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_b)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &40u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::PathInputOutputMismatch.name(),
                            error_code_number: crate::ErrorCode::PathInputOutputMismatch.into(),
                            error_msg: crate::ErrorCode::PathInputOutputMismatch.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 40u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            {
                let __key_a = &input_account.owner;
                let __key_b = &continuation.owner;
                let __account_a = ::vipers::AsKeyRef::as_key_ref(__key_a);
                let __account_b = ::vipers::AsKeyRef::as_key_ref(__key_b);
                if __account_a != __account_b {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &crate::ErrorCode::InputOwnerMismatch,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::InputOwnerMismatch,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("input_account.owner != continuation.owner");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Left: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_a)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Right: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_b)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &45u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::InputOwnerMismatch.name(),
                            error_code_number: crate::ErrorCode::InputOwnerMismatch.into(),
                            error_msg: crate::ErrorCode::InputOwnerMismatch.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 45u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            {
                let __key_a = &input_account.mint;
                let __key_b = &continuation.amount_in.mint;
                let __account_a = ::vipers::AsKeyRef::as_key_ref(__key_a);
                let __account_b = ::vipers::AsKeyRef::as_key_ref(__key_b);
                if __account_a != __account_b {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &crate::ErrorCode::InputMintMismatch,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::InputMintMismatch,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(
                        "input_account.mint != continuation.amount_in.mint",
                    );
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Left: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_a)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Right: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_b)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &46u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::InputMintMismatch.name(),
                            error_code_number: crate::ErrorCode::InputMintMismatch.into(),
                            error_msg: crate::ErrorCode::InputMintMismatch.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 46u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            let amount_in = continuation.amount_in;
            {
                if !(amount_in.amount != 0) {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&crate::ErrorCode::ZeroSwap),
                                ::core::fmt::ArgumentV1::new_display(&crate::ErrorCode::ZeroSwap),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("amount_in.amount != 0");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &54u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::ZeroSwap.name(),
                            error_code_number: crate::ErrorCode::ZeroSwap.into(),
                            error_msg: crate::ErrorCode::ZeroSwap.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 54u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            {
                if !(input_account.amount >= amount_in.amount) {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &crate::ErrorCode::InsufficientInputBalance,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::InsufficientInputBalance,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("input_account.amount >= amount_in.amount");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &57u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::InsufficientInputBalance.name(),
                            error_code_number: crate::ErrorCode::InsufficientInputBalance.into(),
                            error_msg: crate::ErrorCode::InsufficientInputBalance.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 57u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            let output_account = self.output_account();
            {
                let __key_a = &output_account.owner;
                let __key_b = &continuation.owner;
                let __account_a = ::vipers::AsKeyRef::as_key_ref(__key_a);
                let __account_b = ::vipers::AsKeyRef::as_key_ref(__key_b);
                if __account_a != __account_b {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &crate::ErrorCode::OutputOwnerMismatch,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::OutputOwnerMismatch,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("output_account.owner != continuation.owner");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Left: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_a)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Right: "],
                            &[::core::fmt::ArgumentV1::new_display(&__account_b)],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &64u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::OutputOwnerMismatch.name(),
                            error_code_number: crate::ErrorCode::OutputOwnerMismatch.into(),
                            error_msg: crate::ErrorCode::OutputOwnerMismatch.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 64u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            let initial_balance = output_account.amount;
            let minimum_amount_out = if continuation.steps_left == 1 {
                {
                    let __key_a = &continuation.minimum_amount_out.mint;
                    let __key_b = &output_account.mint;
                    let __account_a = ::vipers::AsKeyRef::as_key_ref(__key_a);
                    let __account_b = ::vipers::AsKeyRef::as_key_ref(__key_b);
                    if __account_a != __account_b {
                        ::solana_program::log::sol_log(&*{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_debug(
                                        &crate::ErrorCode::OutputMintMismatch,
                                    ),
                                    ::core::fmt::ArgumentV1::new_display(
                                        &crate::ErrorCode::OutputMintMismatch,
                                    ),
                                ],
                            ));
                            res
                        });
                        ::solana_program::log::sol_log(
                            "continuation.minimum_amount_out.mint != output_account.mint",
                        );
                        ::solana_program::log::sol_log(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Left: "],
                                &[::core::fmt::ArgumentV1::new_display(&__account_a)],
                            ));
                            res
                        });
                        ::solana_program::log::sol_log(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Right: "],
                                &[::core::fmt::ArgumentV1::new_display(&__account_b)],
                            ));
                            res
                        });
                        ::solana_program::log::sol_log(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Error thrown at ", ":"],
                                &match (&"programs/continuation-router/src/processor.rs", &73u32) {
                                    args => [
                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                    ],
                                },
                            ));
                            res
                        });
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::AnchorError {
                                error_name: crate::ErrorCode::OutputMintMismatch.name(),
                                error_code_number: crate::ErrorCode::OutputMintMismatch.into(),
                                error_msg: crate::ErrorCode::OutputMintMismatch.to_string(),
                                error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                    anchor_lang::error::Source {
                                        filename: "programs/continuation-router/src/processor.rs",
                                        line: 73u32,
                                    },
                                )),
                                compared_values: None,
                            },
                        ));
                    }
                };
                continuation.minimum_amount_out.amount
            } else {
                0
            };
            self.process_unchecked(amount_in.amount, minimum_amount_out)?;
            let output_account = &mut output_account.clone();
            output_account.reload()?;
            let result_balance = output_account.amount;
            {
                if !(result_balance >= initial_balance) {
                    ::solana_program::log::sol_log(&*{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&crate::ErrorCode::BalanceLower),
                                ::core::fmt::ArgumentV1::new_display(
                                    &crate::ErrorCode::BalanceLower,
                                ),
                            ],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("result_balance >= initial_balance");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/processor.rs", &88u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: crate::ErrorCode::BalanceLower.name(),
                            error_code_number: crate::ErrorCode::BalanceLower.into(),
                            error_msg: crate::ErrorCode::BalanceLower.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/continuation-router/src/processor.rs",
                                    line: 88u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
            };
            let next_amount_in = result_balance - initial_balance;
            continuation.input = output_account.key();
            continuation.amount_in = TokenAmount::new(output_account.mint, next_amount_in);
            continuation.steps_left -= 1;
            {
                anchor_lang::solana_program::log::sol_log_data(&[&anchor_lang::Event::data(
                    &SwapActionEvent {
                        action_type: Self::TYPE,
                        owner: continuation.owner,
                        input_amount: amount_in,
                        output_account: continuation.input,
                        output_amount: continuation.amount_in,
                    },
                )]);
            };
            Ok(())
        }
    }
}
use crate::action::ProcessAction;
use crate::processor::{ActionContext, Processor};
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        176u8, 55u8, 206u8, 219u8, 51u8, 181u8, 150u8, 16u8, 146u8, 32u8, 78u8, 212u8, 183u8, 92u8,
        196u8, 149u8, 1u8, 210u8, 194u8, 54u8, 254u8, 181u8, 104u8, 251u8, 153u8, 43u8, 185u8,
        47u8, 136u8, 71u8, 249u8, 27u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use self::continuation_router::*;
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| {
        e.log();
        e.into()
    })
}
fn try_entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct ContinuationRouter;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ContinuationRouter {
        #[inline]
        fn clone(&self) -> ContinuationRouter {
            match *self {
                ContinuationRouter => ContinuationRouter,
            }
        }
    }
    impl anchor_lang::Id for ContinuationRouter {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [106, 191, 176, 28, 152, 6, 176, 42] => {
            __private::__global::create_ata_if_not_exists(program_id, accounts, ix_data)
        }
        [35, 150, 31, 42, 98, 141, 116, 108] => {
            __private::__global::begin(program_id, accounts, ix_data)
        }
        [241, 188, 208, 225, 254, 43, 38, 241] => {
            __private::__global::begin_v2(program_id, accounts, ix_data)
        }
        [180, 160, 249, 217, 194, 121, 70, 16] => {
            __private::__global::end(program_id, accounts, ix_data)
        }
        [77, 170, 215, 67, 182, 47, 249, 225] => {
            __private::__global::ss_swap(program_id, accounts, ix_data)
        }
        [101, 22, 53, 15, 236, 31, 246, 84] => {
            __private::__global::ss_withdraw_one(program_id, accounts, ix_data)
        }
        [38, 68, 65, 68, 218, 131, 229, 202] => {
            __private::__global::ss_deposit_a(program_id, accounts, ix_data)
        }
        [109, 222, 231, 0, 90, 222, 238, 233] => {
            __private::__global::ss_deposit_b(program_id, accounts, ix_data)
        }
        [45, 60, 152, 4, 191, 159, 160, 243] => {
            __private::__global::ad_withdraw(program_id, accounts, ix_data)
        }
        [202, 34, 43, 64, 127, 205, 158, 75] => {
            __private::__global::ad_deposit(program_id, accounts, ix_data)
        }
        [49, 251, 76, 149, 148, 65, 229, 158] => {
            __private::__global::__dummy_swap_token(program_id, accounts, ix_data)
        }
        [34, 173, 34, 58, 60, 156, 226, 183] => {
            __private::__global::__dummy_swap_output(program_id, accounts, ix_data)
        }
        [66, 165, 97, 157, 220, 76, 187, 75] => {
            __private::__global::__dummy_ss_swap(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create_ata_if_not_exists(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateAtaIfNotExists");
            let ix = instruction::CreateAtaIfNotExists::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateAtaIfNotExists = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CreateATAIfNotExists::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result =
                continuation_router::create_ata_if_not_exists(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn begin(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Begin");
            let ix = instruction::Begin::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Begin {
                amount_in,
                minimum_amount_out,
                num_steps,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Begin::try_accounts(program_id, &mut remaining_accounts, ix_data, &mut __bumps)?;
            let result = continuation_router::begin(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                amount_in,
                minimum_amount_out,
                num_steps,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn begin_v2(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: BeginV2");
            let ix = instruction::BeginV2::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::BeginV2 {
                amount_in,
                minimum_amount_out,
                num_steps,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                BeginV2::try_accounts(program_id, &mut remaining_accounts, ix_data, &mut __bumps)?;
            let result = continuation_router::begin_v2(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                amount_in,
                minimum_amount_out,
                num_steps,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn end(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: End");
            let ix = instruction::End::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::End = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                End::try_accounts(program_id, &mut remaining_accounts, ix_data, &mut __bumps)?;
            let result = continuation_router::end(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn ss_swap(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SsSwap");
            let ix = instruction::SsSwap::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SsSwap = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SSSwapAccounts::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = continuation_router::ss_swap(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn ss_withdraw_one(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SsWithdrawOne");
            let ix = instruction::SsWithdrawOne::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SsWithdrawOne = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SSWithdrawOneAccounts::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = continuation_router::ss_withdraw_one(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn ss_deposit_a(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SsDepositA");
            let ix = instruction::SsDepositA::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SsDepositA = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SSDepositAAccounts::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = continuation_router::ss_deposit_a(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn ss_deposit_b(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SsDepositB");
            let ix = instruction::SsDepositB::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SsDepositB = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SSDepositBAccounts::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = continuation_router::ss_deposit_b(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn ad_withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: AdWithdraw");
            let ix = instruction::AdWithdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::AdWithdraw = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = ADWithdrawAccounts::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = continuation_router::ad_withdraw(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn ad_deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: AdDeposit");
            let ix = instruction::AdDeposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::AdDeposit = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = ADDepositAccounts::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = continuation_router::ad_deposit(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn __dummy_swap_token(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: DummySwapToken");
            let ix = instruction::DummySwapToken::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DummySwapToken = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SwapToken::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result =
                continuation_router::__dummy_swap_token(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn __dummy_swap_output(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: DummySwapOutput");
            let ix = instruction::DummySwapOutput::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DummySwapOutput = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SwapOutput::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result =
                continuation_router::__dummy_swap_output(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn __dummy_ss_swap(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: DummySsSwap");
            let ix = instruction::DummySsSwap::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DummySsSwap = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SSSwap::try_accounts(program_id, &mut remaining_accounts, ix_data, &mut __bumps)?;
            let result = continuation_router::__dummy_ss_swap(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
    }
}
pub mod continuation_router {
    use super::*;
    /// Creates an ATA if it does not yet exist.
    pub fn create_ata_if_not_exists(ctx: Context<CreateATAIfNotExists>) -> Result<()> {
        if !ctx.accounts.ata.try_borrow_data()?.is_empty() {
            return Ok(());
        }
        anchor_spl::associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;
        Ok(())
    }
    /// Begins a swap transaction.
    pub fn begin(
        ctx: Context<Begin>,
        amount_in: u64,
        minimum_amount_out: u64,
        num_steps: u16,
    ) -> Result<()> {
        let continuation = &mut ctx.accounts.continuation;
        continuation.owner = *ctx.accounts.owner.key;
        continuation.payer = *ctx.accounts.payer.key;
        continuation.input = *ctx.accounts.input.to_account_info().key;
        continuation.initial_amount_in = TokenAmount::new(ctx.accounts.input.mint, amount_in);
        continuation.output = *ctx.accounts.output.to_account_info().key;
        continuation.output_initial_balance = ctx.accounts.output.amount;
        continuation.amount_in = TokenAmount::new(ctx.accounts.input.mint, amount_in);
        continuation.minimum_amount_out =
            TokenAmount::new(ctx.accounts.output.mint, minimum_amount_out);
        continuation.steps_left = num_steps;
        continuation.__nonce =
            *ctx.bumps
                .get("continuation")
                .ok_or_else(|| -> anchor_lang::error::Error {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Option unwrap failed: "],
                            &[::core::fmt::ArgumentV1::new_debug(
                                &::vipers::VipersError::IntegerOverflow,
                            )],
                        ));
                        res
                    });
                    ::solana_program::log::sol_log("ctx.bumps.get(\"continuation\")");
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/continuation-router/src/lib.rs", &85u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ::vipers::VipersError::IntegerOverflow.name(),
                        error_code_number: ::vipers::VipersError::IntegerOverflow.into(),
                        error_msg: ::vipers::VipersError::IntegerOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/continuation-router/src/lib.rs",
                                line: 85u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
        Ok(())
    }
    /// Begins a swap transaction.
    /// More optimized.
    pub fn begin_v2(
        ctx: Context<BeginV2>,
        amount_in: u64,
        minimum_amount_out: u64,
        num_steps: u16,
    ) -> Result<()> {
        let continuation = &mut ctx.accounts.continuation;
        continuation.owner = ctx.accounts.owner.key();
        continuation.payer = ctx.accounts.owner.key();
        continuation.input = ctx.accounts.input.key();
        continuation.initial_amount_in = TokenAmount::new(ctx.accounts.input.mint, amount_in);
        continuation.output = ctx.accounts.output.key();
        continuation.output_initial_balance = ctx.accounts.output.amount;
        continuation.amount_in = TokenAmount::new(ctx.accounts.input.mint, amount_in);
        continuation.minimum_amount_out =
            TokenAmount::new(ctx.accounts.output.mint, minimum_amount_out);
        continuation.steps_left = num_steps;
        Ok(())
    }
    /// Cleans up the transaction and checks several invariants.
    pub fn end(ctx: Context<End>) -> Result<()> {
        let continuation = &ctx.accounts.continuation;
        if !(continuation.steps_left == 0) {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::AnchorError {
                    error_name: crate::ErrorCode::EndIncomplete.name(),
                    error_code_number: crate::ErrorCode::EndIncomplete.into(),
                    error_msg: crate::ErrorCode::EndIncomplete.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/continuation-router/src/lib.rs",
                            line: 117u32,
                        },
                    )),
                    compared_values: None,
                },
            ));
        };
        let result_balance = ctx.accounts.output.amount;
        if !(result_balance >= continuation.output_initial_balance) {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::AnchorError {
                    error_name: crate::ErrorCode::BalanceLower.name(),
                    error_code_number: crate::ErrorCode::BalanceLower.into(),
                    error_msg: crate::ErrorCode::BalanceLower.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/continuation-router/src/lib.rs",
                            line: 120u32,
                        },
                    )),
                    compared_values: None,
                },
            ));
        };
        if !(ctx.accounts.output.mint == continuation.minimum_amount_out.mint) {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::AnchorError {
                    error_name: crate::ErrorCode::OutputMintMismatch.name(),
                    error_code_number: crate::ErrorCode::OutputMintMismatch.into(),
                    error_msg: crate::ErrorCode::OutputMintMismatch.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/continuation-router/src/lib.rs",
                            line: 124u32,
                        },
                    )),
                    compared_values: None,
                },
            ));
        };
        let mut amount_out = result_balance - continuation.output_initial_balance;
        if continuation.initial_amount_in.mint == ctx.accounts.output.mint {
            amount_out += continuation.initial_amount_in.amount;
        }
        if !(amount_out >= continuation.minimum_amount_out.amount) {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::AnchorError {
                    error_name: crate::ErrorCode::MinimumOutNotMet.name(),
                    error_code_number: crate::ErrorCode::MinimumOutNotMet.into(),
                    error_msg: crate::ErrorCode::MinimumOutNotMet.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/continuation-router/src/lib.rs",
                            line: 135u32,
                        },
                    )),
                    compared_values: None,
                },
            ));
        };
        {
            anchor_lang::solana_program::log::sol_log_data(&[&anchor_lang::Event::data(
                &SwapCompleteEvent {
                    owner: continuation.owner,
                    amount_in: continuation.initial_amount_in,
                    amount_out: TokenAmount::new(continuation.minimum_amount_out.mint, amount_out),
                },
            )]);
        };
        Ok(())
    }
    pub fn ss_swap<'info>(ctx: Context<'_, '_, '_, 'info, SSSwapAccounts<'info>>) -> Result<()> {
        {
            let ctx = ctx;
            let cont = &mut ctx.accounts.continuation.continuation;
            let action = &ctx.accounts.action;
            let action_ctx = &ActionContext {
                program_id: ctx.program_id,
                action,
                remaining_accounts: ctx.remaining_accounts,
                token_program: ctx.accounts.continuation.token_program.clone(),
                swap_program: ctx.accounts.continuation.swap_program.to_account_info(),
                owner: ctx.accounts.continuation.owner.to_account_info(),
            };
            Processor::process(action_ctx, cont)
        }
    }
    pub fn ss_withdraw_one<'info>(
        ctx: Context<'_, '_, '_, 'info, SSWithdrawOneAccounts<'info>>,
    ) -> Result<()> {
        {
            let ctx = ctx;
            let cont = &mut ctx.accounts.continuation.continuation;
            let action = &ctx.accounts.action;
            let action_ctx = &ActionContext {
                program_id: ctx.program_id,
                action,
                remaining_accounts: ctx.remaining_accounts,
                token_program: ctx.accounts.continuation.token_program.clone(),
                swap_program: ctx.accounts.continuation.swap_program.to_account_info(),
                owner: ctx.accounts.continuation.owner.to_account_info(),
            };
            Processor::process(action_ctx, cont)
        }
    }
    pub fn ss_deposit_a<'info>(
        ctx: Context<'_, '_, '_, 'info, SSDepositAAccounts<'info>>,
    ) -> Result<()> {
        {
            let ctx = ctx;
            let cont = &mut ctx.accounts.continuation.continuation;
            let action = &ctx.accounts.action;
            let action_ctx = &ActionContext {
                program_id: ctx.program_id,
                action,
                remaining_accounts: ctx.remaining_accounts,
                token_program: ctx.accounts.continuation.token_program.clone(),
                swap_program: ctx.accounts.continuation.swap_program.to_account_info(),
                owner: ctx.accounts.continuation.owner.to_account_info(),
            };
            Processor::process(action_ctx, cont)
        }
    }
    pub fn ss_deposit_b<'info>(
        ctx: Context<'_, '_, '_, 'info, SSDepositBAccounts<'info>>,
    ) -> Result<()> {
        {
            let ctx = ctx;
            let cont = &mut ctx.accounts.continuation.continuation;
            let action = &ctx.accounts.action;
            let action_ctx = &ActionContext {
                program_id: ctx.program_id,
                action,
                remaining_accounts: ctx.remaining_accounts,
                token_program: ctx.accounts.continuation.token_program.clone(),
                swap_program: ctx.accounts.continuation.swap_program.to_account_info(),
                owner: ctx.accounts.continuation.owner.to_account_info(),
            };
            Processor::process(action_ctx, cont)
        }
    }
    pub fn ad_withdraw<'info>(
        ctx: Context<'_, '_, '_, 'info, ADWithdrawAccounts<'info>>,
    ) -> Result<()> {
        {
            let ctx = ctx;
            let cont = &mut ctx.accounts.continuation.continuation;
            let action = &ctx.accounts.action;
            let action_ctx = &ActionContext {
                program_id: ctx.program_id,
                action,
                remaining_accounts: ctx.remaining_accounts,
                token_program: ctx.accounts.continuation.token_program.clone(),
                swap_program: ctx.accounts.continuation.swap_program.to_account_info(),
                owner: ctx.accounts.continuation.owner.to_account_info(),
            };
            Processor::process(action_ctx, cont)
        }
    }
    pub fn ad_deposit<'info>(
        ctx: Context<'_, '_, '_, 'info, ADDepositAccounts<'info>>,
    ) -> Result<()> {
        {
            let ctx = ctx;
            let cont = &mut ctx.accounts.continuation.continuation;
            let action = &ctx.accounts.action;
            let action_ctx = &ActionContext {
                program_id: ctx.program_id,
                action,
                remaining_accounts: ctx.remaining_accounts,
                token_program: ctx.accounts.continuation.token_program.clone(),
                swap_program: ctx.accounts.continuation.swap_program.to_account_info(),
                owner: ctx.accounts.continuation.owner.to_account_info(),
            };
            Processor::process(action_ctx, cont)
        }
    }
    pub fn __dummy_swap_token<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToken<'info>>,
    ) -> Result<()> {
        Ok(())
    }
    pub fn __dummy_swap_output<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapOutput<'info>>,
    ) -> Result<()> {
        Ok(())
    }
    pub fn __dummy_ss_swap<'info>(ctx: Context<'_, '_, '_, 'info, SSSwap<'info>>) -> Result<()> {
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct CreateAtaIfNotExists;
    impl borsh::ser::BorshSerialize for CreateAtaIfNotExists {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateAtaIfNotExists {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for CreateAtaIfNotExists {
        fn data(&self) -> Vec<u8> {
            let mut d = [106, 191, 176, 28, 152, 6, 176, 42].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Begin {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
        pub num_steps: u16,
    }
    impl borsh::ser::BorshSerialize for Begin
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount_in, writer)?;
            borsh::BorshSerialize::serialize(&self.minimum_amount_out, writer)?;
            borsh::BorshSerialize::serialize(&self.num_steps, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Begin
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount_in: borsh::BorshDeserialize::deserialize(buf)?,
                minimum_amount_out: borsh::BorshDeserialize::deserialize(buf)?,
                num_steps: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Begin {
        fn data(&self) -> Vec<u8> {
            let mut d = [35, 150, 31, 42, 98, 141, 116, 108].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct BeginV2 {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
        pub num_steps: u16,
    }
    impl borsh::ser::BorshSerialize for BeginV2
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u16: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount_in, writer)?;
            borsh::BorshSerialize::serialize(&self.minimum_amount_out, writer)?;
            borsh::BorshSerialize::serialize(&self.num_steps, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for BeginV2
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u16: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount_in: borsh::BorshDeserialize::deserialize(buf)?,
                minimum_amount_out: borsh::BorshDeserialize::deserialize(buf)?,
                num_steps: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for BeginV2 {
        fn data(&self) -> Vec<u8> {
            let mut d = [241, 188, 208, 225, 254, 43, 38, 241].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct End;
    impl borsh::ser::BorshSerialize for End {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for End {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for End {
        fn data(&self) -> Vec<u8> {
            let mut d = [180, 160, 249, 217, 194, 121, 70, 16].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SsSwap;
    impl borsh::ser::BorshSerialize for SsSwap {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SsSwap {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SsSwap {
        fn data(&self) -> Vec<u8> {
            let mut d = [77, 170, 215, 67, 182, 47, 249, 225].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SsWithdrawOne;
    impl borsh::ser::BorshSerialize for SsWithdrawOne {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SsWithdrawOne {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SsWithdrawOne {
        fn data(&self) -> Vec<u8> {
            let mut d = [101, 22, 53, 15, 236, 31, 246, 84].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SsDepositA;
    impl borsh::ser::BorshSerialize for SsDepositA {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SsDepositA {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SsDepositA {
        fn data(&self) -> Vec<u8> {
            let mut d = [38, 68, 65, 68, 218, 131, 229, 202].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SsDepositB;
    impl borsh::ser::BorshSerialize for SsDepositB {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SsDepositB {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SsDepositB {
        fn data(&self) -> Vec<u8> {
            let mut d = [109, 222, 231, 0, 90, 222, 238, 233].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct AdWithdraw;
    impl borsh::ser::BorshSerialize for AdWithdraw {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AdWithdraw {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for AdWithdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [45, 60, 152, 4, 191, 159, 160, 243].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct AdDeposit;
    impl borsh::ser::BorshSerialize for AdDeposit {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AdDeposit {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for AdDeposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [202, 34, 43, 64, 127, 205, 158, 75].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DummySwapToken;
    impl borsh::ser::BorshSerialize for DummySwapToken {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DummySwapToken {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for DummySwapToken {
        fn data(&self) -> Vec<u8> {
            let mut d = [49, 251, 76, 149, 148, 65, 229, 158].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DummySwapOutput;
    impl borsh::ser::BorshSerialize for DummySwapOutput {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DummySwapOutput {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for DummySwapOutput {
        fn data(&self) -> Vec<u8> {
            let mut d = [34, 173, 34, 58, 60, 156, 226, 183].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DummySsSwap;
    impl borsh::ser::BorshSerialize for DummySsSwap {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DummySsSwap {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for DummySsSwap {
        fn data(&self) -> Vec<u8> {
            let mut d = [66, 165, 97, 157, 220, 76, 187, 75].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
#[cfg(feature = "cpi")]
pub mod cpi {
    use super::*;
    use std::marker::PhantomData;
    pub mod state {
        use super::*;
    }
    pub struct Return<T> {
        phantom: std::marker::PhantomData<T>,
    }
    impl<T: AnchorDeserialize> Return<T> {
        pub fn get(&self) -> T {
            let (_key, data) = anchor_lang::solana_program::program::get_return_data().unwrap();
            T::try_from_slice(&data).unwrap()
        }
    }
    pub fn create_ata_if_not_exists<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::CreateATAIfNotExists<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::CreateAtaIfNotExists;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [106, 191, 176, 28, 152, 6, 176, 42].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn begin<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::Begin<'info>,
        >,
        amount_in: u64,
        minimum_amount_out: u64,
        num_steps: u16,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::Begin {
                amount_in,
                minimum_amount_out,
                num_steps,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [35, 150, 31, 42, 98, 141, 116, 108].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn begin_v2<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::BeginV2<'info>,
        >,
        amount_in: u64,
        minimum_amount_out: u64,
        num_steps: u16,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::BeginV2 {
                amount_in,
                minimum_amount_out,
                num_steps,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [241, 188, 208, 225, 254, 43, 38, 241].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn end<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<'a, 'b, 'c, 'info, crate::cpi::accounts::End<'info>>,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::End;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [180, 160, 249, 217, 194, 121, 70, 16].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn ss_swap<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SSSwapAccounts<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::SsSwap;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [77, 170, 215, 67, 182, 47, 249, 225].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn ss_withdraw_one<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SSWithdrawOneAccounts<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::SsWithdrawOne;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [101, 22, 53, 15, 236, 31, 246, 84].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn ss_deposit_a<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SSDepositAAccounts<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::SsDepositA;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [38, 68, 65, 68, 218, 131, 229, 202].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn ss_deposit_b<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SSDepositBAccounts<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::SsDepositB;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [109, 222, 231, 0, 90, 222, 238, 233].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn ad_withdraw<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::ADWithdrawAccounts<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::AdWithdraw;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [45, 60, 152, 4, 191, 159, 160, 243].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn ad_deposit<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::ADDepositAccounts<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::AdDeposit;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [202, 34, 43, 64, 127, 205, 158, 75].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn __dummy_swap_token<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SwapToken<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::DummySwapToken;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [49, 251, 76, 149, 148, 65, 229, 158].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn __dummy_swap_output<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SwapOutput<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::DummySwapOutput;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [34, 173, 34, 58, 60, 156, 226, 183].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    pub fn __dummy_ss_swap<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::SSSwap<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::DummySsSwap;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [66, 165, 97, 157, 220, 76, 187, 75].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }
    /// An Anchor generated module, providing a set of structs
    /// mirroring the structs deriving `Accounts`, where each field is
    /// an `AccountInfo`. This is useful for CPI.
    pub mod accounts {
        pub use crate::__cpi_client_accounts_create_ata_if_not_exists::*;
        pub use crate::__cpi_client_accounts_ad_deposit_accounts::*;
        pub use crate::__cpi_client_accounts_end::*;
        pub use crate::__cpi_client_accounts_ss_deposit_a_accounts::*;
        pub use crate::__cpi_client_accounts_swap_output::*;
        pub use crate::__cpi_client_accounts_swap_token::*;
        pub use crate::__cpi_client_accounts_begin_v2::*;
        pub use crate::__cpi_client_accounts_ss_swap_accounts::*;
        pub use crate::__cpi_client_accounts_ss_withdraw_one_accounts::*;
        pub use crate::__cpi_client_accounts_ad_withdraw_accounts::*;
        pub use crate::__cpi_client_accounts_ss_swap::*;
        pub use crate::__cpi_client_accounts_ss_deposit_b_accounts::*;
        pub use crate::__cpi_client_accounts_begin::*;
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_create_ata_if_not_exists::*;
    pub use crate::__client_accounts_begin::*;
    pub use crate::__client_accounts_end::*;
    pub use crate::__client_accounts_ss_swap_accounts::*;
    pub use crate::__client_accounts_ss_deposit_b_accounts::*;
    pub use crate::__client_accounts_ad_deposit_accounts::*;
    pub use crate::__client_accounts_swap_token::*;
    pub use crate::__client_accounts_swap_output::*;
    pub use crate::__client_accounts_ss_swap::*;
    pub use crate::__client_accounts_ss_withdraw_one_accounts::*;
    pub use crate::__client_accounts_begin_v2::*;
    pub use crate::__client_accounts_ss_deposit_a_accounts::*;
    pub use crate::__client_accounts_ad_withdraw_accounts::*;
}
pub struct SSSwap<'info> {
    /// Swap and authority
    pub swap: StableSwap<'info>,
    /// The input token of this component of the route.
    pub input: SwapToken<'info>,
    /// The output token of this component of the route.
    pub output: SwapOutput<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSSwap<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let swap: StableSwap<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let input: SwapToken<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let output: SwapOutput<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSSwap {
            swap,
            input,
            output,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSSwap<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.swap.to_account_infos());
        account_infos.extend(self.input.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSSwap<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.swap.to_account_metas(None));
        account_metas.extend(self.input.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSSwap<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.swap, program_id)
            .map_err(|e| e.with_account_name("swap"))?;
        anchor_lang::AccountsExit::exit(&self.input, program_id)
            .map_err(|e| e.with_account_name("input"))?;
        anchor_lang::AccountsExit::exit(&self.output, program_id)
            .map_err(|e| e.with_account_name("output"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_swap {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_stable_swap::StableSwap;
    pub use __client_accounts_swap_token::SwapToken;
    pub use __client_accounts_swap_output::SwapOutput;
    /// Generated client accounts for [`SSSwap`].
    pub struct SSSwap {
        /// Swap and authority
        pub swap: __client_accounts_stable_swap::StableSwap,
        /// The input token of this component of the route.
        pub input: __client_accounts_swap_token::SwapToken,
        /// The output token of this component of the route.
        pub output: __client_accounts_swap_output::SwapOutput,
    }
    impl borsh::ser::BorshSerialize for SSSwap
    where
        __client_accounts_stable_swap::StableSwap: borsh::ser::BorshSerialize,
        __client_accounts_swap_token::SwapToken: borsh::ser::BorshSerialize,
        __client_accounts_swap_output::SwapOutput: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.swap, writer)?;
            borsh::BorshSerialize::serialize(&self.input, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSSwap {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.swap.to_account_metas(None));
            account_metas.extend(self.input.to_account_metas(None));
            account_metas.extend(self.output.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_swap {
    use super::*;
    pub use __cpi_client_accounts_swap_output::SwapOutput;
    pub use __cpi_client_accounts_stable_swap::StableSwap;
    pub use __cpi_client_accounts_swap_token::SwapToken;
    /// Generated CPI struct of the accounts for [`SSSwap`].
    pub struct SSSwap<'info> {
        /// Swap and authority
        pub swap: __cpi_client_accounts_stable_swap::StableSwap<'info>,
        /// The input token of this component of the route.
        pub input: __cpi_client_accounts_swap_token::SwapToken<'info>,
        /// The output token of this component of the route.
        pub output: __cpi_client_accounts_swap_output::SwapOutput<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSSwap<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.swap.to_account_metas(None));
            account_metas.extend(self.input.to_account_metas(None));
            account_metas.extend(self.output.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSSwap<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.swap));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.input));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.output));
            account_infos
        }
    }
}
impl<'info> crate::processor::ActionInputOutput<'info>
    for ActionContext<'_, '_, '_, 'info, SSSwap<'info>>
{
    fn input_account(&self) -> &Account<'info, TokenAccount> {
        self.action.input_account()
    }
    fn output_account(&self) -> &Account<'info, TokenAccount> {
        self.action.output_account()
    }
}
impl<'info> crate::processor::Processor<'info> for ActionContext<'_, '_, '_, 'info, SSSwap<'info>> {
    fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        ProcessAction::process(self, amount_in, minimum_amount_out)
    }
}
impl<'info> crate::Action for ActionContext<'_, '_, '_, '_, SSSwap<'info>> {
    const TYPE: crate::ActionType = crate::ActionType::SSSwap;
}
pub struct SSWithdrawOne<'info> {
    /// Swap and authority
    pub swap: StableSwap<'info>,
    /// The pool mint of the swap.
    /// CHECK: Checked by [stable_swap_anchor] program.
    #[account(mut)]
    pub pool_mint: AccountInfo<'info>,
    /// The input account for LP tokens.
    #[account(mut)]
    pub input_lp: Account<'info, TokenAccount>,
    /// The output of the unused token of this component of the route.
    /// CHECK: Checked by [stable_swap_anchor] program.
    #[account(mut)]
    pub quote_reserves: AccountInfo<'info>,
    /// The output of this component of the route.
    pub output: SwapOutput<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSWithdrawOne<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let swap: StableSwap<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let pool_mint: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("pool_mint"))?;
        let input_lp: anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("input_lp"))?;
        let quote_reserves: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("quote_reserves"))?;
        let output: SwapOutput<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        if !pool_mint.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("pool_mint"));
        }
        if !input_lp.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("input_lp"));
        }
        if !quote_reserves.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("quote_reserves"));
        }
        Ok(SSWithdrawOne {
            swap,
            pool_mint,
            input_lp,
            quote_reserves,
            output,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSWithdrawOne<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.swap.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.input_lp.to_account_infos());
        account_infos.extend(self.quote_reserves.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSWithdrawOne<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.swap.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.input_lp.to_account_metas(None));
        account_metas.extend(self.quote_reserves.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSWithdrawOne<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.swap, program_id)
            .map_err(|e| e.with_account_name("swap"))?;
        anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)
            .map_err(|e| e.with_account_name("pool_mint"))?;
        anchor_lang::AccountsExit::exit(&self.input_lp, program_id)
            .map_err(|e| e.with_account_name("input_lp"))?;
        anchor_lang::AccountsExit::exit(&self.quote_reserves, program_id)
            .map_err(|e| e.with_account_name("quote_reserves"))?;
        anchor_lang::AccountsExit::exit(&self.output, program_id)
            .map_err(|e| e.with_account_name("output"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_withdraw_one {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_stable_swap::StableSwap;
    pub use __client_accounts_swap_output::SwapOutput;
    /// Generated client accounts for [`SSWithdrawOne`].
    pub struct SSWithdrawOne {
        /// Swap and authority
        pub swap: __client_accounts_stable_swap::StableSwap,
        /// The pool mint of the swap.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        /// The input account for LP tokens.
        pub input_lp: anchor_lang::solana_program::pubkey::Pubkey,
        /// The output of the unused token of this component of the route.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub quote_reserves: anchor_lang::solana_program::pubkey::Pubkey,
        /// The output of this component of the route.
        pub output: __client_accounts_swap_output::SwapOutput,
    }
    impl borsh::ser::BorshSerialize for SSWithdrawOne
    where
        __client_accounts_stable_swap::StableSwap: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_swap_output::SwapOutput: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.swap, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.input_lp, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_reserves, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSWithdrawOne {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.swap.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pool_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.input_lp,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.quote_reserves,
                false,
            ));
            account_metas.extend(self.output.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_withdraw_one {
    use super::*;
    pub use __cpi_client_accounts_swap_output::SwapOutput;
    pub use __cpi_client_accounts_stable_swap::StableSwap;
    /// Generated CPI struct of the accounts for [`SSWithdrawOne`].
    pub struct SSWithdrawOne<'info> {
        /// Swap and authority
        pub swap: __cpi_client_accounts_stable_swap::StableSwap<'info>,
        /// The pool mint of the swap.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The input account for LP tokens.
        pub input_lp: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The output of the unused token of this component of the route.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub quote_reserves: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The output of this component of the route.
        pub output: __cpi_client_accounts_swap_output::SwapOutput<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSWithdrawOne<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.swap.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pool_mint),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.input_lp),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.quote_reserves),
                false,
            ));
            account_metas.extend(self.output.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSWithdrawOne<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.swap));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool_mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.input_lp));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.quote_reserves,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.output));
            account_infos
        }
    }
}
impl<'info> crate::processor::ActionInputOutput<'info>
    for ActionContext<'_, '_, '_, 'info, SSWithdrawOne<'info>>
{
    fn input_account(&self) -> &Account<'info, TokenAccount> {
        self.action.input_account()
    }
    fn output_account(&self) -> &Account<'info, TokenAccount> {
        self.action.output_account()
    }
}
impl<'info> crate::processor::Processor<'info>
    for ActionContext<'_, '_, '_, 'info, SSWithdrawOne<'info>>
{
    fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        ProcessAction::process(self, amount_in, minimum_amount_out)
    }
}
impl<'info> crate::Action for ActionContext<'_, '_, '_, '_, SSWithdrawOne<'info>> {
    const TYPE: crate::ActionType = crate::ActionType::SSWithdrawOne;
}
pub struct SSDepositA<'info> {
    pub inner: SSDeposit<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSDepositA<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let inner: SSDeposit<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSDepositA { inner })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositA<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.inner.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSDepositA<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.inner.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSDepositA<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.inner, program_id)
            .map_err(|e| e.with_account_name("inner"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_deposit_a {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_ss_deposit::SSDeposit;
    /// Generated client accounts for [`SSDepositA`].
    pub struct SSDepositA {
        pub inner: __client_accounts_ss_deposit::SSDeposit,
    }
    impl borsh::ser::BorshSerialize for SSDepositA
    where
        __client_accounts_ss_deposit::SSDeposit: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.inner, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSDepositA {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.inner.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_deposit_a {
    use super::*;
    pub use __cpi_client_accounts_ss_deposit::SSDeposit;
    /// Generated CPI struct of the accounts for [`SSDepositA`].
    pub struct SSDepositA<'info> {
        pub inner: __cpi_client_accounts_ss_deposit::SSDeposit<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSDepositA<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.inner.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositA<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.inner));
            account_infos
        }
    }
}
impl<'info> crate::processor::ActionInputOutput<'info>
    for ActionContext<'_, '_, '_, 'info, SSDepositA<'info>>
{
    fn input_account(&self) -> &Account<'info, TokenAccount> {
        self.action.input_account()
    }
    fn output_account(&self) -> &Account<'info, TokenAccount> {
        self.action.output_account()
    }
}
impl<'info> crate::processor::Processor<'info>
    for ActionContext<'_, '_, '_, 'info, SSDepositA<'info>>
{
    fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        ProcessAction::process(self, amount_in, minimum_amount_out)
    }
}
impl<'info> crate::Action for ActionContext<'_, '_, '_, '_, SSDepositA<'info>> {
    const TYPE: crate::ActionType = crate::ActionType::SSDepositA;
}
pub struct SSDepositB<'info> {
    pub inner: SSDeposit<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSDepositB<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let inner: SSDeposit<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSDepositB { inner })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositB<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.inner.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSDepositB<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.inner.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSDepositB<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.inner, program_id)
            .map_err(|e| e.with_account_name("inner"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_deposit_b {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_ss_deposit::SSDeposit;
    /// Generated client accounts for [`SSDepositB`].
    pub struct SSDepositB {
        pub inner: __client_accounts_ss_deposit::SSDeposit,
    }
    impl borsh::ser::BorshSerialize for SSDepositB
    where
        __client_accounts_ss_deposit::SSDeposit: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.inner, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSDepositB {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.inner.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_deposit_b {
    use super::*;
    pub use __cpi_client_accounts_ss_deposit::SSDeposit;
    /// Generated CPI struct of the accounts for [`SSDepositB`].
    pub struct SSDepositB<'info> {
        pub inner: __cpi_client_accounts_ss_deposit::SSDeposit<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSDepositB<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.inner.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositB<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.inner));
            account_infos
        }
    }
}
impl<'info> crate::processor::ActionInputOutput<'info>
    for ActionContext<'_, '_, '_, 'info, SSDepositB<'info>>
{
    fn input_account(&self) -> &Account<'info, TokenAccount> {
        self.action.input_account()
    }
    fn output_account(&self) -> &Account<'info, TokenAccount> {
        self.action.output_account()
    }
}
impl<'info> crate::processor::Processor<'info>
    for ActionContext<'_, '_, '_, 'info, SSDepositB<'info>>
{
    fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        ProcessAction::process(self, amount_in, minimum_amount_out)
    }
}
impl<'info> crate::Action for ActionContext<'_, '_, '_, '_, SSDepositB<'info>> {
    const TYPE: crate::ActionType = crate::ActionType::SSDepositB;
}
pub struct ADWithdraw<'info> {
    pub input: Account<'info, TokenAccount>,
    pub output: Account<'info, TokenAccount>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ADWithdraw<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let input: anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("input"))?;
        let output: anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("output"))?;
        Ok(ADWithdraw { input, output })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ADWithdraw<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.input.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ADWithdraw<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.input.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ADWithdraw<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ad_withdraw {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`ADWithdraw`].
    pub struct ADWithdraw {
        pub input: anchor_lang::solana_program::pubkey::Pubkey,
        pub output: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ADWithdraw
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.input, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ADWithdraw {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.input, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.output,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ad_withdraw {
    use super::*;
    /// Generated CPI struct of the accounts for [`ADWithdraw`].
    pub struct ADWithdraw<'info> {
        pub input: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub output: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ADWithdraw<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.input),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.output),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ADWithdraw<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.input));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.output));
            account_infos
        }
    }
}
impl<'info> crate::processor::ActionInputOutput<'info>
    for ActionContext<'_, '_, '_, 'info, ADWithdraw<'info>>
{
    fn input_account(&self) -> &Account<'info, TokenAccount> {
        &self.action.input
    }
    fn output_account(&self) -> &Account<'info, TokenAccount> {
        &self.action.output
    }
}
impl<'info> crate::processor::Processor<'info>
    for ActionContext<'_, '_, '_, 'info, ADWithdraw<'info>>
{
    fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        crate::router_action_processor::process_action(
            CpiContext::new(self.swap_program.clone(), self.remaining_accounts.to_vec()),
            Self::TYPE.into(),
            amount_in,
            minimum_amount_out,
        )
    }
}
impl<'info> crate::Action for ActionContext<'_, '_, '_, '_, ADWithdraw<'info>> {
    const TYPE: crate::ActionType = crate::ActionType::ADWithdraw;
}
pub struct ADDeposit<'info> {
    pub input: Account<'info, TokenAccount>,
    pub output: Account<'info, TokenAccount>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ADDeposit<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let input: anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("input"))?;
        let output: anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("output"))?;
        Ok(ADDeposit { input, output })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ADDeposit<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.input.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ADDeposit<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.input.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ADDeposit<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ad_deposit {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`ADDeposit`].
    pub struct ADDeposit {
        pub input: anchor_lang::solana_program::pubkey::Pubkey,
        pub output: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ADDeposit
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.input, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ADDeposit {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.input, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.output,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ad_deposit {
    use super::*;
    /// Generated CPI struct of the accounts for [`ADDeposit`].
    pub struct ADDeposit<'info> {
        pub input: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub output: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ADDeposit<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.input),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.output),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ADDeposit<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.input));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.output));
            account_infos
        }
    }
}
impl<'info> crate::processor::ActionInputOutput<'info>
    for ActionContext<'_, '_, '_, 'info, ADDeposit<'info>>
{
    fn input_account(&self) -> &Account<'info, TokenAccount> {
        &self.action.input
    }
    fn output_account(&self) -> &Account<'info, TokenAccount> {
        &self.action.output
    }
}
impl<'info> crate::processor::Processor<'info>
    for ActionContext<'_, '_, '_, 'info, ADDeposit<'info>>
{
    fn process_unchecked(&self, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        crate::router_action_processor::process_action(
            CpiContext::new(self.swap_program.clone(), self.remaining_accounts.to_vec()),
            Self::TYPE.into(),
            amount_in,
            minimum_amount_out,
        )
    }
}
impl<'info> crate::Action for ActionContext<'_, '_, '_, '_, ADDeposit<'info>> {
    const TYPE: crate::ActionType = crate::ActionType::ADDeposit;
}
/// Token accounts for the destination of a [StableSwap] instruction.
pub struct CreateATAIfNotExists<'info> {
    /// The token accounts of the user and the token.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// The ATA to create.
    #[account(mut)]
    pub ata: SystemAccount<'info>,
    /// Authority of the created ATA.
    /// CHECK: Passed to ATA program.
    pub authority: UncheckedAccount<'info>,
    /// Mint.
    /// CHECK: Not necessary to deserialize.
    pub mint: UncheckedAccount<'info>,
    /// Rent.
    pub rent: Sysvar<'info, Rent>,
    /// System program.
    pub system_program: Program<'info, System>,
    /// Token program.
    pub token_program: Program<'info, Token>,
    /// The associated token program.
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for CreateATAIfNotExists<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let payer: Signer =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("payer"))?;
        let ata: SystemAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("ata"))?;
        let authority: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("authority"))?;
        let mint: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("mint"))?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("rent"))?;
        let system_program: anchor_lang::accounts::program::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("system_program"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("token_program"))?;
        let associated_token_program: anchor_lang::accounts::program::Program<
            anchor_spl::associated_token::AssociatedToken,
        > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
            .map_err(|e| e.with_account_name("associated_token_program"))?;
        if !payer.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("payer"));
        }
        if !ata.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("ata"));
        }
        Ok(CreateATAIfNotExists {
            payer,
            ata,
            authority,
            mint,
            rent,
            system_program,
            token_program,
            associated_token_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateATAIfNotExists<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.payer.to_account_infos());
        account_infos.extend(self.ata.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.associated_token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CreateATAIfNotExists<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.payer.to_account_metas(None));
        account_metas.extend(self.ata.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.associated_token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CreateATAIfNotExists<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.payer, program_id)
            .map_err(|e| e.with_account_name("payer"))?;
        anchor_lang::AccountsExit::exit(&self.ata, program_id)
            .map_err(|e| e.with_account_name("ata"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_create_ata_if_not_exists {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`CreateATAIfNotExists`].
    pub struct CreateATAIfNotExists {
        /// The token accounts of the user and the token.
        pub payer: anchor_lang::solana_program::pubkey::Pubkey,
        /// The ATA to create.
        pub ata: anchor_lang::solana_program::pubkey::Pubkey,
        /// Authority of the created ATA.
        /// CHECK: Passed to ATA program.
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        /// Mint.
        /// CHECK: Not necessary to deserialize.
        pub mint: anchor_lang::solana_program::pubkey::Pubkey,
        /// Rent.
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        /// System program.
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
        /// Token program.
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        /// The associated token program.
        pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateATAIfNotExists
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.payer, writer)?;
            borsh::BorshSerialize::serialize(&self.ata, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.associated_token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CreateATAIfNotExists {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer, true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.ata, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.mint, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.associated_token_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_create_ata_if_not_exists {
    use super::*;
    /// Generated CPI struct of the accounts for [`CreateATAIfNotExists`].
    pub struct CreateATAIfNotExists<'info> {
        /// The token accounts of the user and the token.
        pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The ATA to create.
        pub ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Authority of the created ATA.
        /// CHECK: Passed to ATA program.
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Mint.
        /// CHECK: Not necessary to deserialize.
        pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Rent.
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// System program.
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Token program.
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The associated token program.
        pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateATAIfNotExists<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.payer),
                true,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.ata),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.authority),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.mint),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.associated_token_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateATAIfNotExists<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.ata));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.associated_token_program,
            ));
            account_infos
        }
    }
}
/// Begins a route.
pub struct Begin<'info> {
    /// Continuation state.
    # [account (init , seeds = [b"anchor" . as_ref () , owner . key () . as_ref () , random . key () . as_ref ()] , bump , space = 8 + Continuation :: LEN , payer = payer)]
    pub continuation: Box<Account<'info, Continuation>>,
    /// Nonce used for associating the continuation. Any arbitrary [Pubkey] can be passed here.
    /// CHECK: Arbitrary.
    pub random: UncheckedAccount<'info>,
    /// Input token account.
    # [account (has_one = owner)]
    pub input: Box<Account<'info, TokenAccount>>,
    /// Output token account.
    # [account (has_one = owner)]
    pub output: Box<Account<'info, TokenAccount>>,
    /// Owner of all token accounts in the chain.
    pub owner: Signer<'info>,
    /// Funds the continuation in the beginning transaction and receives
    /// the staked lamports of the continuation in the end transaction.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Rent sysvar.
    pub rent: Sysvar<'info, Rent>,
    /// System program.
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Begin<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        if accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let continuation = &accounts[0];
        *accounts = &accounts[1..];
        let random: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("random"))?;
        let input: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("input"))?;
        let output: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("output"))?;
        let owner: Signer =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("owner"))?;
        let payer: Signer =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("payer"))?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("rent"))?;
        let system_program: anchor_lang::accounts::program::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[
                b"anchor".as_ref(),
                owner.key().as_ref(),
                random.key().as_ref(),
            ],
            program_id,
        );
        __bumps.insert("continuation".to_string(), __bump);
        let continuation = {
            let actual_field = continuation.to_account_info();
            let actual_owner = actual_field.owner;
            let space = 8 + Continuation::LEN;
            let pa: Box<anchor_lang::accounts::account::Account<Continuation>> =
                if !false || actual_owner == &anchor_lang::solana_program::system_program::ID {
                    let payer = payer.to_account_info();
                    let __current_lamports = continuation.lamports();
                    if __current_lamports == 0 {
                        let lamports = __anchor_rent.minimum_balance(space);
                        let cpi_accounts = anchor_lang::system_program::CreateAccount {
                            from: payer.to_account_info(),
                            to: continuation.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::create_account(
                            cpi_context.with_signer(&[&[
                                b"anchor".as_ref(),
                                owner.key().as_ref(),
                                random.key().as_ref(),
                                &[__bump][..],
                            ][..]]),
                            lamports,
                            space as u64,
                            program_id,
                        )?;
                    } else {
                        let required_lamports = __anchor_rent
                            .minimum_balance(space)
                            .max(1)
                            .saturating_sub(__current_lamports);
                        if required_lamports > 0 {
                            let cpi_accounts = anchor_lang::system_program::Transfer {
                                from: payer.to_account_info(),
                                to: continuation.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::transfer(cpi_context, required_lamports)?;
                        }
                        let cpi_accounts = anchor_lang::system_program::Allocate {
                            account_to_allocate: continuation.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::allocate(
                            cpi_context.with_signer(&[&[
                                b"anchor".as_ref(),
                                owner.key().as_ref(),
                                random.key().as_ref(),
                                &[__bump][..],
                            ][..]]),
                            space as u64,
                        )?;
                        let cpi_accounts = anchor_lang::system_program::Assign {
                            account_to_assign: continuation.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::assign(
                            cpi_context.with_signer(&[&[
                                b"anchor".as_ref(),
                                owner.key().as_ref(),
                                random.key().as_ref(),
                                &[__bump][..],
                            ][..]]),
                            program_id,
                        )?;
                    }
                    Box::new(anchor_lang::accounts::account::Account::try_from_unchecked(
                        &continuation,
                    )?)
                } else {
                    Box::new(anchor_lang::accounts::account::Account::try_from(
                        &continuation,
                    )?)
                };
            if false {
                if space != actual_field.data_len() {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSpace,
                    )
                    .with_account_name("continuation")
                    .with_values((space, actual_field.data_len())));
                }
                if actual_owner != program_id {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintOwner,
                    )
                    .with_account_name("continuation")
                    .with_pubkeys((*actual_owner, *program_id)));
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintRentExempt,
                        )
                        .with_account_name("continuation"));
                    }
                }
            }
            pa
        };
        if continuation.key() != __pda_address {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintSeeds,
            )
            .with_account_name("continuation")
            .with_pubkeys((continuation.key(), __pda_address)));
        }
        if !continuation.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("continuation"));
        }
        if !__anchor_rent.is_exempt(
            continuation.to_account_info().lamports(),
            continuation.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintRentExempt,
            )
            .with_account_name("continuation"));
        }
        {
            let my_key = input.owner;
            let target_key = owner.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("input")
                .with_pubkeys((my_key, target_key)));
            }
        }
        {
            let my_key = output.owner;
            let target_key = owner.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("output")
                .with_pubkeys((my_key, target_key)));
            }
        }
        if !payer.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("payer"));
        }
        Ok(Begin {
            continuation,
            random,
            input,
            output,
            owner,
            payer,
            rent,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Begin<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.random.to_account_infos());
        account_infos.extend(self.input.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos.extend(self.owner.to_account_infos());
        account_infos.extend(self.payer.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Begin<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.random.to_account_metas(None));
        account_metas.extend(self.input.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas.extend(self.owner.to_account_metas(None));
        account_metas.extend(self.payer.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Begin<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.payer, program_id)
            .map_err(|e| e.with_account_name("payer"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_begin {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`Begin`].
    pub struct Begin {
        /// Continuation state.
        pub continuation: anchor_lang::solana_program::pubkey::Pubkey,
        /// Nonce used for associating the continuation. Any arbitrary [Pubkey] can be passed here.
        /// CHECK: Arbitrary.
        pub random: anchor_lang::solana_program::pubkey::Pubkey,
        /// Input token account.
        pub input: anchor_lang::solana_program::pubkey::Pubkey,
        /// Output token account.
        pub output: anchor_lang::solana_program::pubkey::Pubkey,
        /// Owner of all token accounts in the chain.
        pub owner: anchor_lang::solana_program::pubkey::Pubkey,
        /// Funds the continuation in the beginning transaction and receives
        /// the staked lamports of the continuation in the end transaction.
        pub payer: anchor_lang::solana_program::pubkey::Pubkey,
        /// Rent sysvar.
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        /// System program.
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Begin
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.random, writer)?;
            borsh::BorshSerialize::serialize(&self.input, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            borsh::BorshSerialize::serialize(&self.owner, writer)?;
            borsh::BorshSerialize::serialize(&self.payer, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Begin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.continuation,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.random,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.input, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.output,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.owner, true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer, true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_begin {
    use super::*;
    /// Generated CPI struct of the accounts for [`Begin`].
    pub struct Begin<'info> {
        /// Continuation state.
        pub continuation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Nonce used for associating the continuation. Any arbitrary [Pubkey] can be passed here.
        /// CHECK: Arbitrary.
        pub random: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Input token account.
        pub input: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Output token account.
        pub output: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Owner of all token accounts in the chain.
        pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Funds the continuation in the beginning transaction and receives
        /// the staked lamports of the continuation in the end transaction.
        pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Rent sysvar.
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// System program.
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Begin<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.continuation),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.random),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.input),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.output),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.owner),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.payer),
                true,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Begin<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.continuation,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.random));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.input));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.output));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
/// Begins a route.
pub struct BeginV2<'info> {
    /// Continuation state.
    #[account(zero)]
    pub continuation: Box<Account<'info, Continuation>>,
    /// Input token account.
    # [account (has_one = owner)]
    pub input: Box<Account<'info, TokenAccount>>,
    /// Output token account.
    # [account (has_one = owner)]
    pub output: Box<Account<'info, TokenAccount>>,
    /// Owner of all token accounts in the chain.
    pub owner: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for BeginV2<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        if accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let continuation = &accounts[0];
        *accounts = &accounts[1..];
        let input: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("input"))?;
        let output: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("output"))?;
        let owner: Signer =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("owner"))?;
        let __anchor_rent = Rent::get()?;
        let continuation: Box<anchor_lang::accounts::account::Account<Continuation>> = {
            let mut __data: &[u8] = &continuation.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintZero,
                )
                .with_account_name("continuation"));
            }
            Box::new(anchor_lang::accounts::account::Account::try_from_unchecked(
                &continuation,
            )?)
        };
        if !continuation.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("continuation"));
        }
        if !__anchor_rent.is_exempt(
            continuation.to_account_info().lamports(),
            continuation.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintRentExempt,
            )
            .with_account_name("continuation"));
        }
        {
            let my_key = input.owner;
            let target_key = owner.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("input")
                .with_pubkeys((my_key, target_key)));
            }
        }
        {
            let my_key = output.owner;
            let target_key = owner.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("output")
                .with_pubkeys((my_key, target_key)));
            }
        }
        Ok(BeginV2 {
            continuation,
            input,
            output,
            owner,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for BeginV2<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.input.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos.extend(self.owner.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for BeginV2<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.input.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas.extend(self.owner.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for BeginV2<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_begin_v2 {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`BeginV2`].
    pub struct BeginV2 {
        /// Continuation state.
        pub continuation: anchor_lang::solana_program::pubkey::Pubkey,
        /// Input token account.
        pub input: anchor_lang::solana_program::pubkey::Pubkey,
        /// Output token account.
        pub output: anchor_lang::solana_program::pubkey::Pubkey,
        /// Owner of all token accounts in the chain.
        pub owner: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for BeginV2
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.input, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            borsh::BorshSerialize::serialize(&self.owner, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for BeginV2 {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.continuation,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.input, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.output,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.owner, true,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_begin_v2 {
    use super::*;
    /// Generated CPI struct of the accounts for [`BeginV2`].
    pub struct BeginV2<'info> {
        /// Continuation state.
        pub continuation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Input token account.
        pub input: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Output token account.
        pub output: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Owner of all token accounts in the chain.
        pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for BeginV2<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.continuation),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.input),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.output),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.owner),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for BeginV2<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.continuation,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.input));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.output));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
            account_infos
        }
    }
}
/// Ends a route.
pub struct End<'info> {
    /// Continuation state.
    # [account (mut , close = payer , has_one = owner , has_one = payer , has_one = output ,)]
    pub continuation: Box<Account<'info, Continuation>>,
    /// Output token account
    pub output: Box<Account<'info, TokenAccount>>,
    /// Owner of all accounts in the chain.
    pub owner: Signer<'info>,
    /// Funds the continuation in the beginning transaction and receives
    /// the staked lamports of the continuation in the end transaction.
    /// CHECK: Arbitrary.
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for End<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: Box<anchor_lang::accounts::account::Account<Continuation>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("continuation"))?;
        let output: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("output"))?;
        let owner: Signer =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("owner"))?;
        let payer: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("payer"))?;
        if !continuation.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("continuation"));
        }
        {
            let my_key = continuation.owner;
            let target_key = owner.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("continuation")
                .with_pubkeys((my_key, target_key)));
            }
        }
        {
            let my_key = continuation.payer;
            let target_key = payer.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("continuation")
                .with_pubkeys((my_key, target_key)));
            }
        }
        {
            let my_key = continuation.output;
            let target_key = output.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("continuation")
                .with_pubkeys((my_key, target_key)));
            }
        }
        if continuation.key() == payer.key() {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintClose,
            )
            .with_account_name("continuation"));
        }
        if !payer.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("payer"));
        }
        Ok(End {
            continuation,
            output,
            owner,
            payer,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for End<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.output.to_account_infos());
        account_infos.extend(self.owner.to_account_infos());
        account_infos.extend(self.payer.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for End<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.output.to_account_metas(None));
        account_metas.extend(self.owner.to_account_metas(None));
        account_metas.extend(self.payer.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for End<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsClose::close(&self.continuation, self.payer.to_account_info())
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.payer, program_id)
            .map_err(|e| e.with_account_name("payer"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_end {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`End`].
    pub struct End {
        /// Continuation state.
        pub continuation: anchor_lang::solana_program::pubkey::Pubkey,
        /// Output token account
        pub output: anchor_lang::solana_program::pubkey::Pubkey,
        /// Owner of all accounts in the chain.
        pub owner: anchor_lang::solana_program::pubkey::Pubkey,
        /// Funds the continuation in the beginning transaction and receives
        /// the staked lamports of the continuation in the end transaction.
        /// CHECK: Arbitrary.
        pub payer: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for End
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.output, writer)?;
            borsh::BorshSerialize::serialize(&self.owner, writer)?;
            borsh::BorshSerialize::serialize(&self.payer, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for End {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.continuation,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.output,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.owner, true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.payer, false,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_end {
    use super::*;
    /// Generated CPI struct of the accounts for [`End`].
    pub struct End<'info> {
        /// Continuation state.
        pub continuation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Output token account
        pub output: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Owner of all accounts in the chain.
        pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// Funds the continuation in the beginning transaction and receives
        /// the staked lamports of the continuation in the end transaction.
        /// CHECK: Arbitrary.
        pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for End<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.continuation),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.output),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.owner),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.payer),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for End<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.continuation,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.output));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
            account_infos
        }
    }
}
pub struct SSSwapAccounts<'info> {
    pub continuation: ContinuationAccounts<'info>,
    pub action: SSSwap<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSSwapAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: ContinuationAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let action: SSSwap<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSSwapAccounts {
            continuation,
            action,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSSwapAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.action.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSSwapAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.action.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSSwapAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.action, program_id)
            .map_err(|e| e.with_account_name("action"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_swap_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __client_accounts_ss_swap::SSSwap;
    /// Generated client accounts for [`SSSwapAccounts`].
    pub struct SSSwapAccounts {
        pub continuation: __client_accounts_continuation_accounts::ContinuationAccounts,
        pub action: __client_accounts_ss_swap::SSSwap,
    }
    impl borsh::ser::BorshSerialize for SSSwapAccounts
    where
        __client_accounts_continuation_accounts::ContinuationAccounts: borsh::ser::BorshSerialize,
        __client_accounts_ss_swap::SSSwap: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.action, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSSwapAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_swap_accounts {
    use super::*;
    pub use __cpi_client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __cpi_client_accounts_ss_swap::SSSwap;
    /// Generated CPI struct of the accounts for [`SSSwapAccounts`].
    pub struct SSSwapAccounts<'info> {
        pub continuation: __cpi_client_accounts_continuation_accounts::ContinuationAccounts<'info>,
        pub action: __cpi_client_accounts_ss_swap::SSSwap<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSSwapAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSSwapAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.continuation,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.action));
            account_infos
        }
    }
}
pub struct SSWithdrawOneAccounts<'info> {
    pub continuation: ContinuationAccounts<'info>,
    pub action: SSWithdrawOne<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSWithdrawOneAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: ContinuationAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let action: SSWithdrawOne<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSWithdrawOneAccounts {
            continuation,
            action,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSWithdrawOneAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.action.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSWithdrawOneAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.action.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSWithdrawOneAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.action, program_id)
            .map_err(|e| e.with_account_name("action"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_withdraw_one_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_ss_withdraw_one::SSWithdrawOne;
    pub use __client_accounts_continuation_accounts::ContinuationAccounts;
    /// Generated client accounts for [`SSWithdrawOneAccounts`].
    pub struct SSWithdrawOneAccounts {
        pub continuation: __client_accounts_continuation_accounts::ContinuationAccounts,
        pub action: __client_accounts_ss_withdraw_one::SSWithdrawOne,
    }
    impl borsh::ser::BorshSerialize for SSWithdrawOneAccounts
    where
        __client_accounts_continuation_accounts::ContinuationAccounts: borsh::ser::BorshSerialize,
        __client_accounts_ss_withdraw_one::SSWithdrawOne: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.action, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSWithdrawOneAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_withdraw_one_accounts {
    use super::*;
    pub use __cpi_client_accounts_ss_withdraw_one::SSWithdrawOne;
    pub use __cpi_client_accounts_continuation_accounts::ContinuationAccounts;
    /// Generated CPI struct of the accounts for [`SSWithdrawOneAccounts`].
    pub struct SSWithdrawOneAccounts<'info> {
        pub continuation: __cpi_client_accounts_continuation_accounts::ContinuationAccounts<'info>,
        pub action: __cpi_client_accounts_ss_withdraw_one::SSWithdrawOne<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSWithdrawOneAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSWithdrawOneAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.continuation,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.action));
            account_infos
        }
    }
}
pub struct SSDepositAAccounts<'info> {
    pub continuation: ContinuationAccounts<'info>,
    pub action: SSDepositA<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSDepositAAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: ContinuationAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let action: SSDepositA<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSDepositAAccounts {
            continuation,
            action,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositAAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.action.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSDepositAAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.action.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSDepositAAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.action, program_id)
            .map_err(|e| e.with_account_name("action"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_deposit_a_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __client_accounts_ss_deposit_a::SSDepositA;
    /// Generated client accounts for [`SSDepositAAccounts`].
    pub struct SSDepositAAccounts {
        pub continuation: __client_accounts_continuation_accounts::ContinuationAccounts,
        pub action: __client_accounts_ss_deposit_a::SSDepositA,
    }
    impl borsh::ser::BorshSerialize for SSDepositAAccounts
    where
        __client_accounts_continuation_accounts::ContinuationAccounts: borsh::ser::BorshSerialize,
        __client_accounts_ss_deposit_a::SSDepositA: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.action, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSDepositAAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_deposit_a_accounts {
    use super::*;
    pub use __cpi_client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __cpi_client_accounts_ss_deposit_a::SSDepositA;
    /// Generated CPI struct of the accounts for [`SSDepositAAccounts`].
    pub struct SSDepositAAccounts<'info> {
        pub continuation: __cpi_client_accounts_continuation_accounts::ContinuationAccounts<'info>,
        pub action: __cpi_client_accounts_ss_deposit_a::SSDepositA<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSDepositAAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositAAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.continuation,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.action));
            account_infos
        }
    }
}
pub struct SSDepositBAccounts<'info> {
    pub continuation: ContinuationAccounts<'info>,
    pub action: SSDepositB<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSDepositBAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: ContinuationAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let action: SSDepositB<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(SSDepositBAccounts {
            continuation,
            action,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositBAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.action.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSDepositBAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.action.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSDepositBAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.action, program_id)
            .map_err(|e| e.with_account_name("action"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_deposit_b_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __client_accounts_ss_deposit_b::SSDepositB;
    /// Generated client accounts for [`SSDepositBAccounts`].
    pub struct SSDepositBAccounts {
        pub continuation: __client_accounts_continuation_accounts::ContinuationAccounts,
        pub action: __client_accounts_ss_deposit_b::SSDepositB,
    }
    impl borsh::ser::BorshSerialize for SSDepositBAccounts
    where
        __client_accounts_continuation_accounts::ContinuationAccounts: borsh::ser::BorshSerialize,
        __client_accounts_ss_deposit_b::SSDepositB: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.action, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSDepositBAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_deposit_b_accounts {
    use super::*;
    pub use __cpi_client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __cpi_client_accounts_ss_deposit_b::SSDepositB;
    /// Generated CPI struct of the accounts for [`SSDepositBAccounts`].
    pub struct SSDepositBAccounts<'info> {
        pub continuation: __cpi_client_accounts_continuation_accounts::ContinuationAccounts<'info>,
        pub action: __cpi_client_accounts_ss_deposit_b::SSDepositB<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSDepositBAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSDepositBAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.continuation,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.action));
            account_infos
        }
    }
}
pub struct ADWithdrawAccounts<'info> {
    pub continuation: ContinuationAccounts<'info>,
    pub action: ADWithdraw<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ADWithdrawAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: ContinuationAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let action: ADWithdraw<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(ADWithdrawAccounts {
            continuation,
            action,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ADWithdrawAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.action.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ADWithdrawAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.action.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ADWithdrawAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.action, program_id)
            .map_err(|e| e.with_account_name("action"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ad_withdraw_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_ad_withdraw::ADWithdraw;
    pub use __client_accounts_continuation_accounts::ContinuationAccounts;
    /// Generated client accounts for [`ADWithdrawAccounts`].
    pub struct ADWithdrawAccounts {
        pub continuation: __client_accounts_continuation_accounts::ContinuationAccounts,
        pub action: __client_accounts_ad_withdraw::ADWithdraw,
    }
    impl borsh::ser::BorshSerialize for ADWithdrawAccounts
    where
        __client_accounts_continuation_accounts::ContinuationAccounts: borsh::ser::BorshSerialize,
        __client_accounts_ad_withdraw::ADWithdraw: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.action, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ADWithdrawAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ad_withdraw_accounts {
    use super::*;
    pub use __cpi_client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __cpi_client_accounts_ad_withdraw::ADWithdraw;
    /// Generated CPI struct of the accounts for [`ADWithdrawAccounts`].
    pub struct ADWithdrawAccounts<'info> {
        pub continuation: __cpi_client_accounts_continuation_accounts::ContinuationAccounts<'info>,
        pub action: __cpi_client_accounts_ad_withdraw::ADWithdraw<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ADWithdrawAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ADWithdrawAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.continuation,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.action));
            account_infos
        }
    }
}
pub struct ADDepositAccounts<'info> {
    pub continuation: ContinuationAccounts<'info>,
    pub action: ADDeposit<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ADDepositAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: ContinuationAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let action: ADDeposit<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        Ok(ADDepositAccounts {
            continuation,
            action,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ADDepositAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.action.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ADDepositAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.action.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ADDepositAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        anchor_lang::AccountsExit::exit(&self.action, program_id)
            .map_err(|e| e.with_account_name("action"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ad_deposit_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __client_accounts_ad_deposit::ADDeposit;
    /// Generated client accounts for [`ADDepositAccounts`].
    pub struct ADDepositAccounts {
        pub continuation: __client_accounts_continuation_accounts::ContinuationAccounts,
        pub action: __client_accounts_ad_deposit::ADDeposit,
    }
    impl borsh::ser::BorshSerialize for ADDepositAccounts
    where
        __client_accounts_continuation_accounts::ContinuationAccounts: borsh::ser::BorshSerialize,
        __client_accounts_ad_deposit::ADDeposit: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.action, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ADDepositAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ad_deposit_accounts {
    use super::*;
    pub use __cpi_client_accounts_continuation_accounts::ContinuationAccounts;
    pub use __cpi_client_accounts_ad_deposit::ADDeposit;
    /// Generated CPI struct of the accounts for [`ADDepositAccounts`].
    pub struct ADDepositAccounts<'info> {
        pub continuation: __cpi_client_accounts_continuation_accounts::ContinuationAccounts<'info>,
        pub action: __cpi_client_accounts_ad_deposit::ADDeposit<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ADDepositAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.continuation.to_account_metas(None));
            account_metas.extend(self.action.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ADDepositAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.continuation,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.action));
            account_infos
        }
    }
}
/// Context common to all router operations.
pub struct ContinuationAccounts<'info> {
    /// Continuation state
    # [account (mut , has_one = owner ,)]
    pub continuation: Box<Account<'info, Continuation>>,
    /// The spl_token program.
    pub token_program: Program<'info, Token>,
    /// The relevant swap program.
    /// CHECK: Arbitrary.
    pub swap_program: UncheckedAccount<'info>,
    /// The owner of all involved token accounts.
    pub owner: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ContinuationAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let continuation: Box<anchor_lang::accounts::account::Account<Continuation>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("continuation"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("token_program"))?;
        let swap_program: UncheckedAccount =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("swap_program"))?;
        let owner: Signer =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("owner"))?;
        if !continuation.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("continuation"));
        }
        {
            let my_key = continuation.owner;
            let target_key = owner.key();
            if my_key != target_key {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                )
                .with_account_name("continuation")
                .with_pubkeys((my_key, target_key)));
            }
        }
        Ok(ContinuationAccounts {
            continuation,
            token_program,
            swap_program,
            owner,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ContinuationAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.continuation.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.swap_program.to_account_infos());
        account_infos.extend(self.owner.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ContinuationAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.continuation.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.swap_program.to_account_metas(None));
        account_metas.extend(self.owner.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ContinuationAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.continuation, program_id)
            .map_err(|e| e.with_account_name("continuation"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_continuation_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`ContinuationAccounts`].
    pub struct ContinuationAccounts {
        /// Continuation state
        pub continuation: anchor_lang::solana_program::pubkey::Pubkey,
        /// The spl_token program.
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        /// The relevant swap program.
        /// CHECK: Arbitrary.
        pub swap_program: anchor_lang::solana_program::pubkey::Pubkey,
        /// The owner of all involved token accounts.
        pub owner: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ContinuationAccounts
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.continuation, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.swap_program, writer)?;
            borsh::BorshSerialize::serialize(&self.owner, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ContinuationAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.continuation,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.swap_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.owner, true,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_continuation_accounts {
    use super::*;
    /// Generated CPI struct of the accounts for [`ContinuationAccounts`].
    pub struct ContinuationAccounts<'info> {
        /// Continuation state
        pub continuation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The spl_token program.
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The relevant swap program.
        /// CHECK: Arbitrary.
        pub swap_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The owner of all involved token accounts.
        pub owner: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ContinuationAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.continuation),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.swap_program),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.owner),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ContinuationAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.continuation,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.swap_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.owner));
            account_infos
        }
    }
}
/// Deposit accounts
pub struct SSDeposit<'info> {
    /// Swap and authority
    pub swap: StableSwap<'info>,
    /// The input of token A of this component of the route.
    pub input_a: SwapToken<'info>,
    /// The input of token B of this component of the route.
    pub input_b: SwapToken<'info>,
    /// The pool mint of the swap.
    /// CHECK: Checked by [stable_swap_anchor] program.
    #[account(mut)]
    pub pool_mint: AccountInfo<'info>,
    /// The destination account for LP tokens.
    #[account(mut)]
    pub output_lp: Account<'info, TokenAccount>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SSDeposit<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let swap: StableSwap<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let input_a: SwapToken<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let input_b: SwapToken<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let pool_mint: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("pool_mint"))?;
        let output_lp: anchor_lang::accounts::account::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("output_lp"))?;
        if !pool_mint.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("pool_mint"));
        }
        if !output_lp.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("output_lp"));
        }
        Ok(SSDeposit {
            swap,
            input_a,
            input_b,
            pool_mint,
            output_lp,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SSDeposit<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.swap.to_account_infos());
        account_infos.extend(self.input_a.to_account_infos());
        account_infos.extend(self.input_b.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.output_lp.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SSDeposit<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.swap.to_account_metas(None));
        account_metas.extend(self.input_a.to_account_metas(None));
        account_metas.extend(self.input_b.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.output_lp.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SSDeposit<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.swap, program_id)
            .map_err(|e| e.with_account_name("swap"))?;
        anchor_lang::AccountsExit::exit(&self.input_a, program_id)
            .map_err(|e| e.with_account_name("input_a"))?;
        anchor_lang::AccountsExit::exit(&self.input_b, program_id)
            .map_err(|e| e.with_account_name("input_b"))?;
        anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)
            .map_err(|e| e.with_account_name("pool_mint"))?;
        anchor_lang::AccountsExit::exit(&self.output_lp, program_id)
            .map_err(|e| e.with_account_name("output_lp"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_ss_deposit {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_stable_swap::StableSwap;
    pub use __client_accounts_swap_token::SwapToken;
    /// Generated client accounts for [`SSDeposit`].
    pub struct SSDeposit {
        /// Swap and authority
        pub swap: __client_accounts_stable_swap::StableSwap,
        /// The input of token A of this component of the route.
        pub input_a: __client_accounts_swap_token::SwapToken,
        /// The input of token B of this component of the route.
        pub input_b: __client_accounts_swap_token::SwapToken,
        /// The pool mint of the swap.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        /// The destination account for LP tokens.
        pub output_lp: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SSDeposit
    where
        __client_accounts_stable_swap::StableSwap: borsh::ser::BorshSerialize,
        __client_accounts_swap_token::SwapToken: borsh::ser::BorshSerialize,
        __client_accounts_swap_token::SwapToken: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.swap, writer)?;
            borsh::BorshSerialize::serialize(&self.input_a, writer)?;
            borsh::BorshSerialize::serialize(&self.input_b, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.output_lp, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SSDeposit {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.swap.to_account_metas(None));
            account_metas.extend(self.input_a.to_account_metas(None));
            account_metas.extend(self.input_b.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pool_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.output_lp,
                false,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_ss_deposit {
    use super::*;
    pub use __cpi_client_accounts_stable_swap::StableSwap;
    pub use __cpi_client_accounts_swap_token::SwapToken;
    /// Generated CPI struct of the accounts for [`SSDeposit`].
    pub struct SSDeposit<'info> {
        /// Swap and authority
        pub swap: __cpi_client_accounts_stable_swap::StableSwap<'info>,
        /// The input of token A of this component of the route.
        pub input_a: __cpi_client_accounts_swap_token::SwapToken<'info>,
        /// The input of token B of this component of the route.
        pub input_b: __cpi_client_accounts_swap_token::SwapToken<'info>,
        /// The pool mint of the swap.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The destination account for LP tokens.
        pub output_lp: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SSDeposit<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.swap.to_account_metas(None));
            account_metas.extend(self.input_a.to_account_metas(None));
            account_metas.extend(self.input_b.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pool_mint),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.output_lp),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SSDeposit<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.swap));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.input_a));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.input_b));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool_mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.output_lp));
            account_infos
        }
    }
}
/// Accounts for interacting with a StableSwap pool.
pub struct StableSwap<'info> {
    /// The swap account
    /// CHECK: Checked by [stable_swap_anchor] program.
    pub swap: AccountInfo<'info>,
    /// The authority of the swap.
    /// CHECK: Checked by [stable_swap_anchor] program.
    pub swap_authority: AccountInfo<'info>,
    /// The clock.
    pub clock: Sysvar<'info, Clock>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for StableSwap<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let swap: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("swap"))?;
        let swap_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("swap_authority"))?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("clock"))?;
        Ok(StableSwap {
            swap,
            swap_authority,
            clock,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for StableSwap<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.swap.to_account_infos());
        account_infos.extend(self.swap_authority.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for StableSwap<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.swap.to_account_metas(None));
        account_metas.extend(self.swap_authority.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for StableSwap<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_stable_swap {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`StableSwap`].
    pub struct StableSwap {
        /// The swap account
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub swap: anchor_lang::solana_program::pubkey::Pubkey,
        /// The authority of the swap.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub swap_authority: anchor_lang::solana_program::pubkey::Pubkey,
        /// The clock.
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for StableSwap
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.swap, writer)?;
            borsh::BorshSerialize::serialize(&self.swap_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for StableSwap {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.swap, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.swap_authority,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.clock, false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_stable_swap {
    use super::*;
    /// Generated CPI struct of the accounts for [`StableSwap`].
    pub struct StableSwap<'info> {
        /// The swap account
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub swap: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The authority of the swap.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub swap_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The clock.
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for StableSwap<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.swap),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.swap_authority),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for StableSwap<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.swap));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.swap_authority,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos
        }
    }
}
/// Token accounts for a [StableSwap] instruction.
pub struct SwapToken<'info> {
    /// The token account associated with the user.
    #[account(mut)]
    pub user: Box<Account<'info, TokenAccount>>,
    /// The token account for the pool's reserves of this token.
    /// CHECK: Checked by [stable_swap_anchor] program.
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SwapToken<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let user: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("user"))?;
        let reserve: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("reserve"))?;
        if !user.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("user"));
        }
        if !reserve.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("reserve"));
        }
        Ok(SwapToken { user, reserve })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SwapToken<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.user.to_account_infos());
        account_infos.extend(self.reserve.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SwapToken<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.user.to_account_metas(None));
        account_metas.extend(self.reserve.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SwapToken<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.user, program_id)
            .map_err(|e| e.with_account_name("user"))?;
        anchor_lang::AccountsExit::exit(&self.reserve, program_id)
            .map_err(|e| e.with_account_name("reserve"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_swap_token {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`SwapToken`].
    pub struct SwapToken {
        /// The token account associated with the user.
        pub user: anchor_lang::solana_program::pubkey::Pubkey,
        /// The token account for the pool's reserves of this token.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SwapToken
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user, writer)?;
            borsh::BorshSerialize::serialize(&self.reserve, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SwapToken {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.user, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.reserve,
                false,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_swap_token {
    use super::*;
    /// Generated CPI struct of the accounts for [`SwapToken`].
    pub struct SwapToken<'info> {
        /// The token account associated with the user.
        pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        /// The token account for the pool's reserves of this token.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SwapToken<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.user),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.reserve),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SwapToken<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.user));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
            account_infos
        }
    }
}
/// Token accounts for the destination of a [StableSwap] instruction.
pub struct SwapOutput<'info> {
    /// The token accounts of the user and the token.
    pub user_token: SwapToken<'info>,
    /// The token account for the fees associated with the token.
    /// CHECK: Checked by [stable_swap_anchor] program.
    #[account(mut)]
    pub fees: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SwapOutput<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
    ) -> anchor_lang::Result<Self> {
        let user_token: SwapToken<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
        let fees: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                .map_err(|e| e.with_account_name("fees"))?;
        if !fees.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("fees"));
        }
        Ok(SwapOutput { user_token, fees })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SwapOutput<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.user_token.to_account_infos());
        account_infos.extend(self.fees.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SwapOutput<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.user_token.to_account_metas(None));
        account_metas.extend(self.fees.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SwapOutput<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.user_token, program_id)
            .map_err(|e| e.with_account_name("user_token"))?;
        anchor_lang::AccountsExit::exit(&self.fees, program_id)
            .map_err(|e| e.with_account_name("fees"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_swap_output {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_swap_token::SwapToken;
    /// Generated client accounts for [`SwapOutput`].
    pub struct SwapOutput {
        /// The token accounts of the user and the token.
        pub user_token: __client_accounts_swap_token::SwapToken,
        /// The token account for the fees associated with the token.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub fees: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SwapOutput
    where
        __client_accounts_swap_token::SwapToken: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user_token, writer)?;
            borsh::BorshSerialize::serialize(&self.fees, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SwapOutput {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.user_token.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.fees, false,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_swap_output {
    use super::*;
    pub use __cpi_client_accounts_swap_token::SwapToken;
    /// Generated CPI struct of the accounts for [`SwapOutput`].
    pub struct SwapOutput<'info> {
        /// The token accounts of the user and the token.
        pub user_token: __cpi_client_accounts_swap_token::SwapToken<'info>,
        /// The token account for the fees associated with the token.
        /// CHECK: Checked by [stable_swap_anchor] program.
        pub fees: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SwapOutput<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.user_token.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.fees),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SwapOutput<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.user_token,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.fees));
            account_infos
        }
    }
}
/// Continuation state of the owner.
pub struct Continuation {
    /// The owner of the continuation.
    pub owner: Pubkey,
    /// The payer of the continuation.
    pub payer: Pubkey,
    /// The initial amount of tokens in.
    pub initial_amount_in: TokenAmount,
    /// The next input account.
    pub input: Pubkey,
    /// The next amount of tokens to input.
    pub amount_in: TokenAmount,
    /// The total number of steps that still need to be executed.
    pub steps_left: u16,
    /// The final output account.
    pub output: Pubkey,
    /// The initial balance of the output account.
    pub output_initial_balance: u64,
    /// The minimum amount of tokens to output at the end of the transaction.
    pub minimum_amount_out: TokenAmount,
    /// Nonce field to the struct to hold the bump seed for the program derived address,
    /// sourced from `<https://github.com/project-serum/anchor/blob/ec6888a3b9f702bc41bd3266e7dd70116df3549c/lang/attribute/account/src/lib.rs#L220-L221.>`.
    __nonce: u8,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Continuation {
    #[inline]
    fn default() -> Continuation {
        Continuation {
            owner: ::core::default::Default::default(),
            payer: ::core::default::Default::default(),
            initial_amount_in: ::core::default::Default::default(),
            input: ::core::default::Default::default(),
            amount_in: ::core::default::Default::default(),
            steps_left: ::core::default::Default::default(),
            output: ::core::default::Default::default(),
            output_initial_balance: ::core::default::Default::default(),
            minimum_amount_out: ::core::default::Default::default(),
            __nonce: ::core::default::Default::default(),
        }
    }
}
impl borsh::ser::BorshSerialize for Continuation
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
    u16: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.owner, writer)?;
        borsh::BorshSerialize::serialize(&self.payer, writer)?;
        borsh::BorshSerialize::serialize(&self.initial_amount_in, writer)?;
        borsh::BorshSerialize::serialize(&self.input, writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, writer)?;
        borsh::BorshSerialize::serialize(&self.steps_left, writer)?;
        borsh::BorshSerialize::serialize(&self.output, writer)?;
        borsh::BorshSerialize::serialize(&self.output_initial_balance, writer)?;
        borsh::BorshSerialize::serialize(&self.minimum_amount_out, writer)?;
        borsh::BorshSerialize::serialize(&self.__nonce, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Continuation
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
    u16: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            owner: borsh::BorshDeserialize::deserialize(buf)?,
            payer: borsh::BorshDeserialize::deserialize(buf)?,
            initial_amount_in: borsh::BorshDeserialize::deserialize(buf)?,
            input: borsh::BorshDeserialize::deserialize(buf)?,
            amount_in: borsh::BorshDeserialize::deserialize(buf)?,
            steps_left: borsh::BorshDeserialize::deserialize(buf)?,
            output: borsh::BorshDeserialize::deserialize(buf)?,
            output_initial_balance: borsh::BorshDeserialize::deserialize(buf)?,
            minimum_amount_out: borsh::BorshDeserialize::deserialize(buf)?,
            __nonce: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Continuation {
    #[inline]
    fn clone(&self) -> Continuation {
        match *self {
            Continuation {
                owner: ref __self_0_0,
                payer: ref __self_0_1,
                initial_amount_in: ref __self_0_2,
                input: ref __self_0_3,
                amount_in: ref __self_0_4,
                steps_left: ref __self_0_5,
                output: ref __self_0_6,
                output_initial_balance: ref __self_0_7,
                minimum_amount_out: ref __self_0_8,
                __nonce: ref __self_0_9,
            } => Continuation {
                owner: ::core::clone::Clone::clone(&(*__self_0_0)),
                payer: ::core::clone::Clone::clone(&(*__self_0_1)),
                initial_amount_in: ::core::clone::Clone::clone(&(*__self_0_2)),
                input: ::core::clone::Clone::clone(&(*__self_0_3)),
                amount_in: ::core::clone::Clone::clone(&(*__self_0_4)),
                steps_left: ::core::clone::Clone::clone(&(*__self_0_5)),
                output: ::core::clone::Clone::clone(&(*__self_0_6)),
                output_initial_balance: ::core::clone::Clone::clone(&(*__self_0_7)),
                minimum_amount_out: ::core::clone::Clone::clone(&(*__self_0_8)),
                __nonce: ::core::clone::Clone::clone(&(*__self_0_9)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Continuation {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
        if writer
            .write_all(&[99, 253, 111, 82, 70, 215, 234, 19])
            .is_err()
        {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        if AnchorSerialize::serialize(self, writer).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Continuation {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [99, 253, 111, 82, 70, 215, 234, 19].len() {
            return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[99, 253, 111, 82, 70, 215, 234, 19] != given_disc {
            return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Continuation {
    fn discriminator() -> [u8; 8] {
        [99, 253, 111, 82, 70, 215, 234, 19]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Continuation {
    fn owner() -> Pubkey {
        crate::ID
    }
}
impl Continuation {
    pub const LEN: usize = PUBKEY_BYTES * 2
        + TokenAmount::LEN
        + PUBKEY_BYTES
        + TokenAmount::LEN
        + 2
        + PUBKEY_BYTES
        + 8
        + TokenAmount::LEN
        + 1;
}
#[repr(u32)]
/// --------------------------------
/// Error codes
/// --------------------------------
pub enum ErrorCode {
    PathInputOutputMismatch,
    TransitiveSwapCalculationError,
    OverflowSwapResult,
    BalanceLower,
    ZeroSwap,
    InputOwnerMismatch,
    InputMintMismatch,
    OutputOwnerMismatch,
    NoMoreSteps,
    InsufficientInputBalance,
    EndIncomplete,
    MinimumOutNotMet,
    OutputMintMismatch,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ErrorCode::PathInputOutputMismatch,) => {
                ::core::fmt::Formatter::write_str(f, "PathInputOutputMismatch")
            }
            (&ErrorCode::TransitiveSwapCalculationError,) => {
                ::core::fmt::Formatter::write_str(f, "TransitiveSwapCalculationError")
            }
            (&ErrorCode::OverflowSwapResult,) => {
                ::core::fmt::Formatter::write_str(f, "OverflowSwapResult")
            }
            (&ErrorCode::BalanceLower,) => ::core::fmt::Formatter::write_str(f, "BalanceLower"),
            (&ErrorCode::ZeroSwap,) => ::core::fmt::Formatter::write_str(f, "ZeroSwap"),
            (&ErrorCode::InputOwnerMismatch,) => {
                ::core::fmt::Formatter::write_str(f, "InputOwnerMismatch")
            }
            (&ErrorCode::InputMintMismatch,) => {
                ::core::fmt::Formatter::write_str(f, "InputMintMismatch")
            }
            (&ErrorCode::OutputOwnerMismatch,) => {
                ::core::fmt::Formatter::write_str(f, "OutputOwnerMismatch")
            }
            (&ErrorCode::NoMoreSteps,) => ::core::fmt::Formatter::write_str(f, "NoMoreSteps"),
            (&ErrorCode::InsufficientInputBalance,) => {
                ::core::fmt::Formatter::write_str(f, "InsufficientInputBalance")
            }
            (&ErrorCode::EndIncomplete,) => ::core::fmt::Formatter::write_str(f, "EndIncomplete"),
            (&ErrorCode::MinimumOutNotMet,) => {
                ::core::fmt::Formatter::write_str(f, "MinimumOutNotMet")
            }
            (&ErrorCode::OutputMintMismatch,) => {
                ::core::fmt::Formatter::write_str(f, "OutputMintMismatch")
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ErrorCode {
    #[inline]
    fn clone(&self) -> ErrorCode {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for ErrorCode {}
impl ErrorCode {
    /// Gets the name of this [#enum_name].
    pub fn name(&self) -> String {
        match self {
            ErrorCode::PathInputOutputMismatch => "PathInputOutputMismatch".to_string(),
            ErrorCode::TransitiveSwapCalculationError => {
                "TransitiveSwapCalculationError".to_string()
            }
            ErrorCode::OverflowSwapResult => "OverflowSwapResult".to_string(),
            ErrorCode::BalanceLower => "BalanceLower".to_string(),
            ErrorCode::ZeroSwap => "ZeroSwap".to_string(),
            ErrorCode::InputOwnerMismatch => "InputOwnerMismatch".to_string(),
            ErrorCode::InputMintMismatch => "InputMintMismatch".to_string(),
            ErrorCode::OutputOwnerMismatch => "OutputOwnerMismatch".to_string(),
            ErrorCode::NoMoreSteps => "NoMoreSteps".to_string(),
            ErrorCode::InsufficientInputBalance => "InsufficientInputBalance".to_string(),
            ErrorCode::EndIncomplete => "EndIncomplete".to_string(),
            ErrorCode::MinimumOutNotMet => "MinimumOutNotMet".to_string(),
            ErrorCode::OutputMintMismatch => "OutputMintMismatch".to_string(),
        }
    }
}
impl From<ErrorCode> for u32 {
    fn from(e: ErrorCode) -> u32 {
        e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
    }
}
impl From<ErrorCode> for anchor_lang::error::Error {
    fn from(error_code: ErrorCode) -> anchor_lang::error::Error {
        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
            error_name: error_code.name(),
            error_code_number: error_code.into(),
            error_msg: error_code.to_string(),
            error_origin: None,
            compared_values: None,
        })
    }
}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::PathInputOutputMismatch => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Path input does not match prior output."],
                &[],
            )),
            ErrorCode::TransitiveSwapCalculationError => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error in a transitive swap input/output calculation."],
                    &[],
                ))
            }
            ErrorCode::OverflowSwapResult => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Swap result overflowed when checking balance difference."],
                &[],
            )),
            ErrorCode::BalanceLower => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Swap resulted in a balance lower than the original balance."],
                &[],
            )),
            ErrorCode::ZeroSwap => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Cannot perform a zero swap."],
                &[],
            )),
            ErrorCode::InputOwnerMismatch => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Input owner does not match continuation owner."],
                &[],
            )),
            ErrorCode::InputMintMismatch => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Input mint does not match continuation input mint."],
                &[],
            )),
            ErrorCode::OutputOwnerMismatch => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Output owner does not match continuation owner."],
                &[],
            )),
            ErrorCode::NoMoreSteps => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["No more steps to process."],
                &[],
            )),
            ErrorCode::InsufficientInputBalance => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Insufficient input balance"],
                &[],
            )),
            ErrorCode::EndIncomplete => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Not all steps were processed."],
                &[],
            )),
            ErrorCode::MinimumOutNotMet => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Minimum amount out not met."],
                &[],
            )),
            ErrorCode::OutputMintMismatch => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Output mint does not match continuation output mint."],
                &[],
            )),
        }
    }
}
pub struct SwapActionEvent {
    pub action_type: ActionType,
    pub owner: Pubkey,
    pub input_amount: TokenAmount,
    pub output_account: Pubkey,
    pub output_amount: TokenAmount,
}
impl borsh::ser::BorshSerialize for SwapActionEvent
where
    ActionType: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.action_type, writer)?;
        borsh::BorshSerialize::serialize(&self.owner, writer)?;
        borsh::BorshSerialize::serialize(&self.input_amount, writer)?;
        borsh::BorshSerialize::serialize(&self.output_account, writer)?;
        borsh::BorshSerialize::serialize(&self.output_amount, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for SwapActionEvent
where
    ActionType: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            action_type: borsh::BorshDeserialize::deserialize(buf)?,
            owner: borsh::BorshDeserialize::deserialize(buf)?,
            input_amount: borsh::BorshDeserialize::deserialize(buf)?,
            output_account: borsh::BorshDeserialize::deserialize(buf)?,
            output_amount: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl anchor_lang::Event for SwapActionEvent {
    fn data(&self) -> Vec<u8> {
        let mut d = [6, 114, 163, 169, 186, 233, 120, 250].to_vec();
        d.append(&mut self.try_to_vec().unwrap());
        d
    }
}
impl anchor_lang::Discriminator for SwapActionEvent {
    fn discriminator() -> [u8; 8] {
        [6, 114, 163, 169, 186, 233, 120, 250]
    }
}
pub struct SwapCompleteEvent {
    pub owner: Pubkey,
    pub amount_in: TokenAmount,
    pub amount_out: TokenAmount,
}
impl borsh::ser::BorshSerialize for SwapCompleteEvent
where
    Pubkey: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
    TokenAmount: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.owner, writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in, writer)?;
        borsh::BorshSerialize::serialize(&self.amount_out, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for SwapCompleteEvent
where
    Pubkey: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
    TokenAmount: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            owner: borsh::BorshDeserialize::deserialize(buf)?,
            amount_in: borsh::BorshDeserialize::deserialize(buf)?,
            amount_out: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl anchor_lang::Event for SwapCompleteEvent {
    fn data(&self) -> Vec<u8> {
        let mut d = [95, 210, 213, 216, 161, 151, 251, 84].to_vec();
        d.append(&mut self.try_to_vec().unwrap());
        d
    }
}
impl anchor_lang::Discriminator for SwapCompleteEvent {
    fn discriminator() -> [u8; 8] {
        [95, 210, 213, 216, 161, 151, 251, 84]
    }
}
/// An amount of tokens.
pub struct TokenAmount {
    /// Mint of the token.
    pub mint: Pubkey,
    /// Amount of the token.
    pub amount: u64,
}
impl borsh::ser::BorshSerialize for TokenAmount
where
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.mint, writer)?;
        borsh::BorshSerialize::serialize(&self.amount, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for TokenAmount
where
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            mint: borsh::BorshDeserialize::deserialize(buf)?,
            amount: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for TokenAmount {
    #[inline]
    fn clone(&self) -> TokenAmount {
        {
            let _: ::core::clone::AssertParamIsClone<Pubkey>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for TokenAmount {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for TokenAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            TokenAmount {
                mint: ref __self_0_0,
                amount: ref __self_0_1,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "TokenAmount");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "mint", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "amount", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for TokenAmount {
    #[inline]
    fn default() -> TokenAmount {
        TokenAmount {
            mint: ::core::default::Default::default(),
            amount: ::core::default::Default::default(),
        }
    }
}
impl ::core::marker::StructuralEq for TokenAmount {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for TokenAmount {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<Pubkey>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
}
impl ::core::marker::StructuralPartialEq for TokenAmount {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for TokenAmount {
    #[inline]
    fn eq(&self, other: &TokenAmount) -> bool {
        match *other {
            TokenAmount {
                mint: ref __self_1_0,
                amount: ref __self_1_1,
            } => match *self {
                TokenAmount {
                    mint: ref __self_0_0,
                    amount: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TokenAmount) -> bool {
        match *other {
            TokenAmount {
                mint: ref __self_1_0,
                amount: ref __self_1_1,
            } => match *self {
                TokenAmount {
                    mint: ref __self_0_0,
                    amount: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
impl TokenAmount {
    pub const LEN: usize = PUBKEY_BYTES + 8;
    fn new(mint: Pubkey, amount: u64) -> TokenAmount {
        TokenAmount { mint, amount }
    }
}
/// An action.
pub trait Action {
    const TYPE: ActionType;
}
/// Interface for programs that can be routed through.
pub trait RouterActionProcessor<'info, T: Accounts<'info>> {
    fn process_action(
        ctx: Context<T>,
        action: u16,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()>;
}
/// Anchor generated module for invoking programs implementing an
/// `#[interface]` via CPI.
mod router_action_processor {
    use super::*;
    pub fn process_action<
        'a,
        'b,
        'c,
        'info,
        T: anchor_lang::Accounts<'info>
            + anchor_lang::ToAccountMetas
            + anchor_lang::ToAccountInfos<'info>,
    >(
        ctx: anchor_lang::context::CpiContext<'a, 'b, 'c, 'info, T>,
        action: u16,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> anchor_lang::Result<()> {
        use anchor_lang::prelude::borsh;
        struct Args {
            action: u16,
            amount_in: u64,
            minimum_amount_out: u64,
        }
        impl borsh::ser::BorshSerialize for Args
        where
            u16: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.action, writer)?;
                borsh::BorshSerialize::serialize(&self.amount_in, writer)?;
                borsh::BorshSerialize::serialize(&self.minimum_amount_out, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Args
        where
            u16: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    action: borsh::BorshDeserialize::deserialize(buf)?,
                    amount_in: borsh::BorshDeserialize::deserialize(buf)?,
                    minimum_amount_out: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let ix = {
            let ix = Args {
                action,
                amount_in,
                minimum_amount_out,
            };
            let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [214, 8, 4, 77, 180, 230, 207, 158].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: *ctx.program.key,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        acc_infos.push(ctx.program.clone());
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_err(Into::into)
    }
}
/// Represents a swap from one token to another.
#[repr(u16)]
pub enum ActionType {
    SSSwap = 0,
    SSWithdrawOne = 1,
    SSDepositA = 2,
    SSDepositB = 3,
    ADWithdraw = 10,
    ADDeposit = 11,
}
impl borsh::ser::BorshSerialize for ActionType {
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> core::result::Result<(), borsh::maybestd::io::Error> {
        let variant_idx: u8 = match self {
            ActionType::SSSwap => 0u8,
            ActionType::SSWithdrawOne => 1u8,
            ActionType::SSDepositA => 2u8,
            ActionType::SSDepositB => 3u8,
            ActionType::ADWithdraw => 4u8,
            ActionType::ADDeposit => 5u8,
        };
        writer.write_all(&variant_idx.to_le_bytes())?;
        match self {
            ActionType::SSSwap => {}
            ActionType::SSWithdrawOne => {}
            ActionType::SSDepositA => {}
            ActionType::SSDepositB => {}
            ActionType::ADWithdraw => {}
            ActionType::ADDeposit => {}
        }
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for ActionType {
    fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
        let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
        let return_value = match variant_idx {
            0u8 => ActionType::SSSwap,
            1u8 => ActionType::SSWithdrawOne,
            2u8 => ActionType::SSDepositA,
            3u8 => ActionType::SSDepositB,
            4u8 => ActionType::ADWithdraw,
            5u8 => ActionType::ADDeposit,
            _ => {
                let msg = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Unexpected variant index: "],
                        &[::core::fmt::ArgumentV1::new_debug(&variant_idx)],
                    ));
                    res
                };
                return Err(borsh::maybestd::io::Error::new(
                    borsh::maybestd::io::ErrorKind::InvalidInput,
                    msg,
                ));
            }
        };
        Ok(return_value)
    }
}
impl From<ActionType> for u16 {
    #[inline]
    fn from(enum_value: ActionType) -> Self {
        enum_value as Self
    }
}
impl ::num_enum::TryFromPrimitive for ActionType {
    type Primitive = u16;
    const NAME: &'static str = "ActionType";
    fn try_from_primitive(
        number: Self::Primitive,
    ) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>> {
        #![allow(non_upper_case_globals)]
        const SSSwap__num_enum_0__: u16 = 0;
        const SSWithdrawOne__num_enum_0__: u16 = 1;
        const SSDepositA__num_enum_0__: u16 = 2;
        const SSDepositB__num_enum_0__: u16 = 3;
        const ADWithdraw__num_enum_0__: u16 = 10;
        const ADDeposit__num_enum_0__: u16 = 11;
        #[deny(unreachable_patterns)]
        match number {
            SSSwap__num_enum_0__ => ::core::result::Result::Ok(Self::SSSwap),
            SSWithdrawOne__num_enum_0__ => ::core::result::Result::Ok(Self::SSWithdrawOne),
            SSDepositA__num_enum_0__ => ::core::result::Result::Ok(Self::SSDepositA),
            SSDepositB__num_enum_0__ => ::core::result::Result::Ok(Self::SSDepositB),
            ADWithdraw__num_enum_0__ => ::core::result::Result::Ok(Self::ADWithdraw),
            ADDeposit__num_enum_0__ => ::core::result::Result::Ok(Self::ADDeposit),
            #[allow(unreachable_patterns)]
            _ => ::core::result::Result::Err(::num_enum::TryFromPrimitiveError { number }),
        }
    }
}
impl ::core::convert::TryFrom<u16> for ActionType {
    type Error = ::num_enum::TryFromPrimitiveError<Self>;
    #[inline]
    fn try_from(
        number: u16,
    ) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>> {
        ::num_enum::TryFromPrimitive::try_from_primitive(number)
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for ActionType {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ActionType {
    #[inline]
    fn clone(&self) -> ActionType {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ActionType::SSSwap,) => ::core::fmt::Formatter::write_str(f, "SSSwap"),
            (&ActionType::SSWithdrawOne,) => ::core::fmt::Formatter::write_str(f, "SSWithdrawOne"),
            (&ActionType::SSDepositA,) => ::core::fmt::Formatter::write_str(f, "SSDepositA"),
            (&ActionType::SSDepositB,) => ::core::fmt::Formatter::write_str(f, "SSDepositB"),
            (&ActionType::ADWithdraw,) => ::core::fmt::Formatter::write_str(f, "ADWithdraw"),
            (&ActionType::ADDeposit,) => ::core::fmt::Formatter::write_str(f, "ADDeposit"),
        }
    }
}
