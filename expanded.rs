#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
pub mod typedefs {
    //! User-defined types.
    use super::*;
    pub struct WithdrawParams {
        pub withdraw_amount: u64,
        pub withdrawal: Withdrawal,
    }
    #[automatically_derived]
    impl ::core::default::Default for WithdrawParams {
        #[inline]
        fn default() -> WithdrawParams {
            WithdrawParams {
                withdraw_amount: ::core::default::Default::default(),
                withdrawal: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WithdrawParams {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "WithdrawParams",
                "withdraw_amount",
                &self.withdraw_amount,
                "withdrawal",
                &&self.withdrawal,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for WithdrawParams {}
    impl borsh::ser::BorshSerialize for WithdrawParams
    where
        u64: borsh::ser::BorshSerialize,
        Withdrawal: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.withdraw_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.withdrawal, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawParams
    where
        u64: borsh::BorshDeserialize,
        Withdrawal: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                withdraw_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                withdrawal: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WithdrawParams {
        #[inline]
        fn clone(&self) -> WithdrawParams {
            WithdrawParams {
                withdraw_amount: ::core::clone::Clone::clone(&self.withdraw_amount),
                withdrawal: ::core::clone::Clone::clone(&self.withdrawal),
            }
        }
    }
    pub enum Withdrawal {
        In,
        Out,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Withdrawal {}
    impl borsh::ser::BorshSerialize for Withdrawal {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 = match self {
                Withdrawal::In => 0u8,
                Withdrawal::Out => 1u8,
            };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                Withdrawal::In => {}
                Withdrawal::Out => {}
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Withdrawal {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for Withdrawal {
        fn deserialize_variant<R: borsh::maybestd::io::Read>(
            reader: &mut R,
            variant_idx: u8,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let mut return_value = match variant_idx {
                0u8 => Withdrawal::In,
                1u8 => Withdrawal::Out,
                _ => {
                    return Err(
                        borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("Unexpected variant index: {0:?}", variant_idx),
                                );
                                res
                            },
                        ),
                    );
                }
            };
            Ok(return_value)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Withdrawal {
        #[inline]
        fn clone(&self) -> Withdrawal {
            match self {
                Withdrawal::In => Withdrawal::In,
                Withdrawal::Out => Withdrawal::Out,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Withdrawal {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Withdrawal::In => "In",
                    Withdrawal::Out => "Out",
                },
            )
        }
    }
    impl Default for Withdrawal {
        fn default() -> Self {
            Self::In
        }
    }
}
pub mod state {
    //! Structs of accounts which hold state.
    use super::*;
    /// Account: Dca
    pub struct Dca {
        pub user: Pubkey,
        pub input_mint: Pubkey,
        pub output_mint: Pubkey,
        pub idx: u64,
        pub next_cycle_at: i64,
        pub in_deposited: u64,
        pub in_withdrawn: u64,
        pub out_withdrawn: u64,
        pub in_used: u64,
        pub out_received: u64,
        pub in_amount_per_cycle: u64,
        pub cycle_frequency: i64,
        pub next_cycle_amount_left: u64,
        pub in_account: Pubkey,
        pub out_account: Pubkey,
        pub min_out_amount: u64,
        pub max_out_amount: u64,
        pub keeper_in_balance_before_borrow: u64,
        pub dca_out_balance_before_swap: u64,
        pub created_at: i64,
        pub bump: u8,
    }
    #[automatically_derived]
    impl ::core::default::Default for Dca {
        #[inline]
        fn default() -> Dca {
            Dca {
                user: ::core::default::Default::default(),
                input_mint: ::core::default::Default::default(),
                output_mint: ::core::default::Default::default(),
                idx: ::core::default::Default::default(),
                next_cycle_at: ::core::default::Default::default(),
                in_deposited: ::core::default::Default::default(),
                in_withdrawn: ::core::default::Default::default(),
                out_withdrawn: ::core::default::Default::default(),
                in_used: ::core::default::Default::default(),
                out_received: ::core::default::Default::default(),
                in_amount_per_cycle: ::core::default::Default::default(),
                cycle_frequency: ::core::default::Default::default(),
                next_cycle_amount_left: ::core::default::Default::default(),
                in_account: ::core::default::Default::default(),
                out_account: ::core::default::Default::default(),
                min_out_amount: ::core::default::Default::default(),
                max_out_amount: ::core::default::Default::default(),
                keeper_in_balance_before_borrow: ::core::default::Default::default(),
                dca_out_balance_before_swap: ::core::default::Default::default(),
                created_at: ::core::default::Default::default(),
                bump: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Dca {}
    impl borsh::ser::BorshSerialize for Dca
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user, writer)?;
            borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.idx, writer)?;
            borsh::BorshSerialize::serialize(&self.next_cycle_at, writer)?;
            borsh::BorshSerialize::serialize(&self.in_deposited, writer)?;
            borsh::BorshSerialize::serialize(&self.in_withdrawn, writer)?;
            borsh::BorshSerialize::serialize(&self.out_withdrawn, writer)?;
            borsh::BorshSerialize::serialize(&self.in_used, writer)?;
            borsh::BorshSerialize::serialize(&self.out_received, writer)?;
            borsh::BorshSerialize::serialize(&self.in_amount_per_cycle, writer)?;
            borsh::BorshSerialize::serialize(&self.cycle_frequency, writer)?;
            borsh::BorshSerialize::serialize(&self.next_cycle_amount_left, writer)?;
            borsh::BorshSerialize::serialize(&self.in_account, writer)?;
            borsh::BorshSerialize::serialize(&self.out_account, writer)?;
            borsh::BorshSerialize::serialize(&self.min_out_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.max_out_amount, writer)?;
            borsh::BorshSerialize::serialize(
                &self.keeper_in_balance_before_borrow,
                writer,
            )?;
            borsh::BorshSerialize::serialize(&self.dca_out_balance_before_swap, writer)?;
            borsh::BorshSerialize::serialize(&self.created_at, writer)?;
            borsh::BorshSerialize::serialize(&self.bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Dca
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                user: borsh::BorshDeserialize::deserialize_reader(reader)?,
                input_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                output_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                idx: borsh::BorshDeserialize::deserialize_reader(reader)?,
                next_cycle_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_deposited: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_withdrawn: borsh::BorshDeserialize::deserialize_reader(reader)?,
                out_withdrawn: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_used: borsh::BorshDeserialize::deserialize_reader(reader)?,
                out_received: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_amount_per_cycle: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                cycle_frequency: borsh::BorshDeserialize::deserialize_reader(reader)?,
                next_cycle_amount_left: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                in_account: borsh::BorshDeserialize::deserialize_reader(reader)?,
                out_account: borsh::BorshDeserialize::deserialize_reader(reader)?,
                min_out_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_out_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                keeper_in_balance_before_borrow: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                dca_out_balance_before_swap: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                created_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Dca {
        #[inline]
        fn clone(&self) -> Dca {
            Dca {
                user: ::core::clone::Clone::clone(&self.user),
                input_mint: ::core::clone::Clone::clone(&self.input_mint),
                output_mint: ::core::clone::Clone::clone(&self.output_mint),
                idx: ::core::clone::Clone::clone(&self.idx),
                next_cycle_at: ::core::clone::Clone::clone(&self.next_cycle_at),
                in_deposited: ::core::clone::Clone::clone(&self.in_deposited),
                in_withdrawn: ::core::clone::Clone::clone(&self.in_withdrawn),
                out_withdrawn: ::core::clone::Clone::clone(&self.out_withdrawn),
                in_used: ::core::clone::Clone::clone(&self.in_used),
                out_received: ::core::clone::Clone::clone(&self.out_received),
                in_amount_per_cycle: ::core::clone::Clone::clone(
                    &self.in_amount_per_cycle,
                ),
                cycle_frequency: ::core::clone::Clone::clone(&self.cycle_frequency),
                next_cycle_amount_left: ::core::clone::Clone::clone(
                    &self.next_cycle_amount_left,
                ),
                in_account: ::core::clone::Clone::clone(&self.in_account),
                out_account: ::core::clone::Clone::clone(&self.out_account),
                min_out_amount: ::core::clone::Clone::clone(&self.min_out_amount),
                max_out_amount: ::core::clone::Clone::clone(&self.max_out_amount),
                keeper_in_balance_before_borrow: ::core::clone::Clone::clone(
                    &self.keeper_in_balance_before_borrow,
                ),
                dca_out_balance_before_swap: ::core::clone::Clone::clone(
                    &self.dca_out_balance_before_swap,
                ),
                created_at: ::core::clone::Clone::clone(&self.created_at),
                bump: ::core::clone::Clone::clone(&self.bump),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Dca {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[82, 93, 90, 127, 40, 101, 145, 154]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Dca {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [82, 93, 90, 127, 40, 101, 145, 154].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[82, 93, 90, 127, 40, 101, 145, 154] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "src/lib.rs",
                                    line: 1u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("Dca"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Dca {
        const DISCRIMINATOR: [u8; 8] = [82, 93, 90, 127, 40, 101, 145, 154];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Dca {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
}
pub mod ix_accounts {
    //! Accounts used in instructions.
    use super::*;
    pub struct OpenDca<'info> {
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub input_mint: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        #[account(mut)]
        pub user_ata: AccountInfo<'info>,
        #[account(mut)]
        pub in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub out_ata: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for OpenDca<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let user: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let user_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_ata"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("out_ata"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !user.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user"),
                );
            }
            if !user_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_ata"),
                );
            }
            if !in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("in_ata"),
                );
            }
            if !out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("out_ata"),
                );
            }
            Ok(OpenDca {
                dca,
                user,
                input_mint,
                output_mint,
                user_ata,
                in_ata,
                out_ata,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for OpenDca<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.user_ata.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.out_ata.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for OpenDca<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.user_ata.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.out_ata.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for OpenDca<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.user, program_id)
                .map_err(|e| e.with_account_name("user"))?;
            anchor_lang::AccountsExit::exit(&self.user_ata, program_id)
                .map_err(|e| e.with_account_name("user_ata"))?;
            anchor_lang::AccountsExit::exit(&self.in_ata, program_id)
                .map_err(|e| e.with_account_name("in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.out_ata, program_id)
                .map_err(|e| e.with_account_name("out_ata"))?;
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
    pub(crate) mod __client_accounts_open_dca {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`OpenDca`].
        pub struct OpenDca {
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for OpenDca
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.user_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for OpenDca {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_open_dca {
        use super::*;
        /// Generated CPI struct of the accounts for [`OpenDca`].
        pub struct OpenDca<'info> {
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub out_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for OpenDca<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for OpenDca<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_ata),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct OpenDcaV2<'info> {
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        pub user: Signer<'info>,
        #[account(mut)]
        pub payer: Signer<'info>,
        pub input_mint: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        #[account(mut)]
        pub user_ata: AccountInfo<'info>,
        #[account(mut)]
        pub in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub out_ata: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for OpenDcaV2<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let user: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let payer: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("payer"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let user_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_ata"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("out_ata"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !payer.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("payer"),
                );
            }
            if !user_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_ata"),
                );
            }
            if !in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("in_ata"),
                );
            }
            if !out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("out_ata"),
                );
            }
            Ok(OpenDcaV2 {
                dca,
                user,
                payer,
                input_mint,
                output_mint,
                user_ata,
                in_ata,
                out_ata,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for OpenDcaV2<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.payer.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.user_ata.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.out_ata.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for OpenDcaV2<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.payer.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.user_ata.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.out_ata.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for OpenDcaV2<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.payer, program_id)
                .map_err(|e| e.with_account_name("payer"))?;
            anchor_lang::AccountsExit::exit(&self.user_ata, program_id)
                .map_err(|e| e.with_account_name("user_ata"))?;
            anchor_lang::AccountsExit::exit(&self.in_ata, program_id)
                .map_err(|e| e.with_account_name("in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.out_ata, program_id)
                .map_err(|e| e.with_account_name("out_ata"))?;
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
    pub(crate) mod __client_accounts_open_dca_v2 {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`OpenDcaV2`].
        pub struct OpenDcaV2 {
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub payer: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for OpenDcaV2
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.payer, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.user_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for OpenDcaV2 {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.payer,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_open_dca_v2 {
        use super::*;
        /// Generated CPI struct of the accounts for [`OpenDcaV2`].
        pub struct OpenDcaV2<'info> {
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub out_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for OpenDcaV2<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.payer),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for OpenDcaV2<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.payer));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_ata),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct CloseDca<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        pub input_mint: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        #[account(mut)]
        pub in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub out_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user_in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user_out_ata: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for CloseDca<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let user: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("out_ata"))?;
            let user_in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_in_ata"))?;
            let user_out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_out_ata"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !user.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("in_ata"),
                );
            }
            if !out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("out_ata"),
                );
            }
            if !user_in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_in_ata"),
                );
            }
            if !user_out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_out_ata"),
                );
            }
            Ok(CloseDca {
                user,
                dca,
                input_mint,
                output_mint,
                in_ata,
                out_ata,
                user_in_ata,
                user_out_ata,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CloseDca<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.out_ata.to_account_infos());
            account_infos.extend(self.user_in_ata.to_account_infos());
            account_infos.extend(self.user_out_ata.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CloseDca<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.out_ata.to_account_metas(None));
            account_metas.extend(self.user_in_ata.to_account_metas(None));
            account_metas.extend(self.user_out_ata.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CloseDca<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.user, program_id)
                .map_err(|e| e.with_account_name("user"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.in_ata, program_id)
                .map_err(|e| e.with_account_name("in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.out_ata, program_id)
                .map_err(|e| e.with_account_name("out_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user_in_ata, program_id)
                .map_err(|e| e.with_account_name("user_in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user_out_ata, program_id)
                .map_err(|e| e.with_account_name("user_out_ata"))?;
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
    pub(crate) mod __client_accounts_close_dca {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`CloseDca`].
        pub struct CloseDca {
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for CloseDca
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user_in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user_out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CloseDca {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_close_dca {
        use super::*;
        /// Generated CPI struct of the accounts for [`CloseDca`].
        pub struct CloseDca<'info> {
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub out_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user_in_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_out_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CloseDca<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CloseDca<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_in_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct Withdraw<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        pub input_mint: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        #[account(mut)]
        pub dca_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user_in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user_out_ata: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for Withdraw<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let user: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let dca_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca_ata"))?;
            let user_in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_in_ata"))?;
            let user_out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_out_ata"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !user.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !dca_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca_ata"),
                );
            }
            if !user_in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_in_ata"),
                );
            }
            if !user_out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_out_ata"),
                );
            }
            Ok(Withdraw {
                user,
                dca,
                input_mint,
                output_mint,
                dca_ata,
                user_in_ata,
                user_out_ata,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.dca_ata.to_account_infos());
            account_infos.extend(self.user_in_ata.to_account_infos());
            account_infos.extend(self.user_out_ata.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.dca_ata.to_account_metas(None));
            account_metas.extend(self.user_in_ata.to_account_metas(None));
            account_metas.extend(self.user_out_ata.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Withdraw<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.user, program_id)
                .map_err(|e| e.with_account_name("user"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.dca_ata, program_id)
                .map_err(|e| e.with_account_name("dca_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user_in_ata, program_id)
                .map_err(|e| e.with_account_name("user_in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user_out_ata, program_id)
                .map_err(|e| e.with_account_name("user_out_ata"))?;
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
    pub(crate) mod __client_accounts_withdraw {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`Withdraw`].
        pub struct Withdraw {
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for Withdraw
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.dca_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user_in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user_out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Withdraw {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_withdraw {
        use super::*;
        /// Generated CPI struct of the accounts for [`Withdraw`].
        pub struct Withdraw<'info> {
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub dca_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user_in_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_out_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.dca_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_in_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct Deposit<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        #[account(mut)]
        pub in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user_in_ata: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for Deposit<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let user: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let user_in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_in_ata"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !user.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("in_ata"),
                );
            }
            if !user_in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_in_ata"),
                );
            }
            Ok(Deposit {
                user,
                dca,
                in_ata,
                user_in_ata,
                token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.user_in_ata.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.user_in_ata.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Deposit<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.user, program_id)
                .map_err(|e| e.with_account_name("user"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.in_ata, program_id)
                .map_err(|e| e.with_account_name("in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user_in_ata, program_id)
                .map_err(|e| e.with_account_name("user_in_ata"))?;
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
    pub(crate) mod __client_accounts_deposit {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`Deposit`].
        pub struct Deposit {
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for Deposit
        where
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
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user_in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Deposit {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_deposit {
        use super::*;
        /// Generated CPI struct of the accounts for [`Deposit`].
        pub struct Deposit<'info> {
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user_in_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_in_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct WithdrawFees<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,
        pub mint: AccountInfo<'info>,
        pub fee_authority: AccountInfo<'info>,
        #[account(mut)]
        pub program_fee_ata: AccountInfo<'info>,
        #[account(mut)]
        pub admin_fee_ata: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for WithdrawFees<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let admin: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("admin"))?;
            let mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("mint"))?;
            let fee_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_authority"))?;
            let program_fee_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program_fee_ata"))?;
            let admin_fee_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("admin_fee_ata"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            if !admin.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("admin"),
                );
            }
            if !program_fee_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("program_fee_ata"),
                );
            }
            if !admin_fee_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("admin_fee_ata"),
                );
            }
            Ok(WithdrawFees {
                admin,
                mint,
                fee_authority,
                program_fee_ata,
                admin_fee_ata,
                system_program,
                token_program,
                associated_token_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawFees<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.admin.to_account_infos());
            account_infos.extend(self.mint.to_account_infos());
            account_infos.extend(self.fee_authority.to_account_infos());
            account_infos.extend(self.program_fee_ata.to_account_infos());
            account_infos.extend(self.admin_fee_ata.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WithdrawFees<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.admin.to_account_metas(None));
            account_metas.extend(self.mint.to_account_metas(None));
            account_metas.extend(self.fee_authority.to_account_metas(None));
            account_metas.extend(self.program_fee_ata.to_account_metas(None));
            account_metas.extend(self.admin_fee_ata.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for WithdrawFees<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.admin, program_id)
                .map_err(|e| e.with_account_name("admin"))?;
            anchor_lang::AccountsExit::exit(&self.program_fee_ata, program_id)
                .map_err(|e| e.with_account_name("program_fee_ata"))?;
            anchor_lang::AccountsExit::exit(&self.admin_fee_ata, program_id)
                .map_err(|e| e.with_account_name("admin_fee_ata"))?;
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
    pub(crate) mod __client_accounts_withdraw_fees {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`WithdrawFees`].
        pub struct WithdrawFees {
            pub admin: anchor_lang::solana_program::pubkey::Pubkey,
            pub mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub fee_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program_fee_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub admin_fee_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for WithdrawFees
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
                borsh::BorshSerialize::serialize(&self.admin, writer)?;
                borsh::BorshSerialize::serialize(&self.mint, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program_fee_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.admin_fee_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for WithdrawFees {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.admin,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.fee_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.program_fee_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.admin_fee_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
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
    pub(crate) mod __cpi_client_accounts_withdraw_fees {
        use super::*;
        /// Generated CPI struct of the accounts for [`WithdrawFees`].
        pub struct WithdrawFees<'info> {
            pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub fee_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program_fee_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub admin_fee_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawFees<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.admin),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.fee_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.program_fee_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.admin_fee_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawFees<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.admin));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.mint));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.program_fee_ata,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.admin_fee_ata,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
            }
        }
    }
    pub struct InitiateFlashFill<'info> {
        #[account(mut)]
        pub keeper: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        pub input_mint: AccountInfo<'info>,
        #[account(mut)]
        pub keeper_in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub in_ata: AccountInfo<'info>,
        pub out_ata: AccountInfo<'info>,
        pub instructions_sysvar: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for InitiateFlashFill<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let keeper: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("keeper"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let keeper_in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("keeper_in_ata"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("out_ata"))?;
            let instructions_sysvar: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("instructions_sysvar"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            if !keeper.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("keeper"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !keeper_in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("keeper_in_ata"),
                );
            }
            if !in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("in_ata"),
                );
            }
            Ok(InitiateFlashFill {
                keeper,
                dca,
                input_mint,
                keeper_in_ata,
                in_ata,
                out_ata,
                instructions_sysvar,
                system_program,
                token_program,
                associated_token_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitiateFlashFill<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.keeper.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.keeper_in_ata.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.out_ata.to_account_infos());
            account_infos.extend(self.instructions_sysvar.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitiateFlashFill<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.keeper.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.keeper_in_ata.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.out_ata.to_account_metas(None));
            account_metas.extend(self.instructions_sysvar.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitiateFlashFill<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.keeper, program_id)
                .map_err(|e| e.with_account_name("keeper"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.keeper_in_ata, program_id)
                .map_err(|e| e.with_account_name("keeper_in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.in_ata, program_id)
                .map_err(|e| e.with_account_name("in_ata"))?;
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
    pub(crate) mod __client_accounts_initiate_flash_fill {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`InitiateFlashFill`].
        pub struct InitiateFlashFill {
            pub keeper: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub keeper_in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub instructions_sysvar: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitiateFlashFill
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.keeper, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.keeper_in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.instructions_sysvar, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitiateFlashFill {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.keeper,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.keeper_in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.instructions_sysvar,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
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
    pub(crate) mod __cpi_client_accounts_initiate_flash_fill {
        use super::*;
        /// Generated CPI struct of the accounts for [`InitiateFlashFill`].
        pub struct InitiateFlashFill<'info> {
            pub keeper: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub keeper_in_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub out_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub instructions_sysvar: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitiateFlashFill<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.keeper),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.keeper_in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.instructions_sysvar),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitiateFlashFill<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.keeper));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.keeper_in_ata,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.instructions_sysvar,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
            }
        }
    }
    pub struct FulfillFlashFill<'info> {
        #[account(mut)]
        pub keeper: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        pub input_mint: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        pub keeper_in_ata: AccountInfo<'info>,
        pub in_ata: AccountInfo<'info>,
        pub out_ata: AccountInfo<'info>,
        pub fee_authority: AccountInfo<'info>,
        #[account(mut)]
        pub fee_ata: AccountInfo<'info>,
        pub instructions_sysvar: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for FulfillFlashFill<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let keeper: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("keeper"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let keeper_in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("keeper_in_ata"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("out_ata"))?;
            let fee_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_authority"))?;
            let fee_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_ata"))?;
            let instructions_sysvar: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("instructions_sysvar"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !keeper.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("keeper"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !fee_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("fee_ata"),
                );
            }
            Ok(FulfillFlashFill {
                keeper,
                dca,
                input_mint,
                output_mint,
                keeper_in_ata,
                in_ata,
                out_ata,
                fee_authority,
                fee_ata,
                instructions_sysvar,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for FulfillFlashFill<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.keeper.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.keeper_in_ata.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.out_ata.to_account_infos());
            account_infos.extend(self.fee_authority.to_account_infos());
            account_infos.extend(self.fee_ata.to_account_infos());
            account_infos.extend(self.instructions_sysvar.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for FulfillFlashFill<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.keeper.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.keeper_in_ata.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.out_ata.to_account_metas(None));
            account_metas.extend(self.fee_authority.to_account_metas(None));
            account_metas.extend(self.fee_ata.to_account_metas(None));
            account_metas.extend(self.instructions_sysvar.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for FulfillFlashFill<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.keeper, program_id)
                .map_err(|e| e.with_account_name("keeper"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.fee_ata, program_id)
                .map_err(|e| e.with_account_name("fee_ata"))?;
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
    pub(crate) mod __client_accounts_fulfill_flash_fill {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`FulfillFlashFill`].
        pub struct FulfillFlashFill {
            pub keeper: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub keeper_in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub fee_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub fee_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub instructions_sysvar: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for FulfillFlashFill
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.keeper, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.keeper_in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.instructions_sysvar, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for FulfillFlashFill {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.keeper,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.keeper_in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.fee_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.fee_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.instructions_sysvar,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_fulfill_flash_fill {
        use super::*;
        /// Generated CPI struct of the accounts for [`FulfillFlashFill`].
        pub struct FulfillFlashFill<'info> {
            pub keeper: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub keeper_in_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub out_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub fee_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub instructions_sysvar: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for FulfillFlashFill<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.keeper),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.keeper_in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.fee_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.fee_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.instructions_sysvar),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for FulfillFlashFill<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.keeper));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.keeper_in_ata,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.fee_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.instructions_sysvar,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct Transfer<'info> {
        #[account(mut)]
        pub keeper: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        #[account(mut)]
        pub user: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        #[account(mut)]
        pub dca_out_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user_out_ata: AccountInfo<'info>,
        #[account(mut)]
        pub intermediate_account: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for Transfer<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let keeper: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("keeper"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let user: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let dca_out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca_out_ata"))?;
            let user_out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_out_ata"))?;
            let intermediate_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("intermediate_account"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !keeper.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("keeper"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !user.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user"),
                );
            }
            if !dca_out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca_out_ata"),
                );
            }
            if !user_out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_out_ata"),
                );
            }
            if !intermediate_account.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("intermediate_account"),
                );
            }
            Ok(Transfer {
                keeper,
                dca,
                user,
                output_mint,
                dca_out_ata,
                user_out_ata,
                intermediate_account,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Transfer<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.keeper.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.dca_out_ata.to_account_infos());
            account_infos.extend(self.user_out_ata.to_account_infos());
            account_infos.extend(self.intermediate_account.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Transfer<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.keeper.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.dca_out_ata.to_account_metas(None));
            account_metas.extend(self.user_out_ata.to_account_metas(None));
            account_metas.extend(self.intermediate_account.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Transfer<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.keeper, program_id)
                .map_err(|e| e.with_account_name("keeper"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.user, program_id)
                .map_err(|e| e.with_account_name("user"))?;
            anchor_lang::AccountsExit::exit(&self.dca_out_ata, program_id)
                .map_err(|e| e.with_account_name("dca_out_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user_out_ata, program_id)
                .map_err(|e| e.with_account_name("user_out_ata"))?;
            anchor_lang::AccountsExit::exit(&self.intermediate_account, program_id)
                .map_err(|e| e.with_account_name("intermediate_account"))?;
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
    pub(crate) mod __client_accounts_transfer {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`Transfer`].
        pub struct Transfer {
            pub keeper: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca_out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub intermediate_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for Transfer
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.keeper, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.dca_out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user_out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.intermediate_account, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Transfer {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.keeper,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca_out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.intermediate_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_transfer {
        use super::*;
        /// Generated CPI struct of the accounts for [`Transfer`].
        pub struct Transfer<'info> {
            pub keeper: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub dca_out_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_out_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub intermediate_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Transfer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.keeper),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca_out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.intermediate_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Transfer<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.keeper));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.dca_out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.intermediate_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
    pub struct EndAndClose<'info> {
        #[account(mut)]
        pub keeper: Signer<'info>,
        #[account(mut)]
        pub dca: AccountInfo<'info>,
        pub input_mint: AccountInfo<'info>,
        pub output_mint: AccountInfo<'info>,
        #[account(mut)]
        pub in_ata: AccountInfo<'info>,
        #[account(mut)]
        pub out_ata: AccountInfo<'info>,
        #[account(mut)]
        pub user: AccountInfo<'info>,
        #[account(mut)]
        pub user_out_ata: AccountInfo<'info>,
        #[account(mut)]
        pub init_user_out_ata: AccountInfo<'info>,
        #[account(mut)]
        pub intermediate_account: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub associated_token_program: AccountInfo<'info>,
        pub event_authority: AccountInfo<'info>,
        pub program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for EndAndClose<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut std::collections::BTreeMap<String, u8>,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let keeper: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("keeper"))?;
            let dca: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("dca"))?;
            let input_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("input_mint"))?;
            let output_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("output_mint"))?;
            let in_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("in_ata"))?;
            let out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("out_ata"))?;
            let user: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user"))?;
            let user_out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_out_ata"))?;
            let init_user_out_ata: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("init_user_out_ata"))?;
            let intermediate_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("intermediate_account"))?;
            let system_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("system_program"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let associated_token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
            let event_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("event_authority"))?;
            let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("program"))?;
            if !keeper.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("keeper"),
                );
            }
            if !dca.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("dca"),
                );
            }
            if !in_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("in_ata"),
                );
            }
            if !out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("out_ata"),
                );
            }
            if !user.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user"),
                );
            }
            if !user_out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_out_ata"),
                );
            }
            if !init_user_out_ata.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("init_user_out_ata"),
                );
            }
            if !intermediate_account.to_account_info().is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("intermediate_account"),
                );
            }
            Ok(EndAndClose {
                keeper,
                dca,
                input_mint,
                output_mint,
                in_ata,
                out_ata,
                user,
                user_out_ata,
                init_user_out_ata,
                intermediate_account,
                system_program,
                token_program,
                associated_token_program,
                event_authority,
                program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for EndAndClose<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.keeper.to_account_infos());
            account_infos.extend(self.dca.to_account_infos());
            account_infos.extend(self.input_mint.to_account_infos());
            account_infos.extend(self.output_mint.to_account_infos());
            account_infos.extend(self.in_ata.to_account_infos());
            account_infos.extend(self.out_ata.to_account_infos());
            account_infos.extend(self.user.to_account_infos());
            account_infos.extend(self.user_out_ata.to_account_infos());
            account_infos.extend(self.init_user_out_ata.to_account_infos());
            account_infos.extend(self.intermediate_account.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.associated_token_program.to_account_infos());
            account_infos.extend(self.event_authority.to_account_infos());
            account_infos.extend(self.program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for EndAndClose<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.keeper.to_account_metas(None));
            account_metas.extend(self.dca.to_account_metas(None));
            account_metas.extend(self.input_mint.to_account_metas(None));
            account_metas.extend(self.output_mint.to_account_metas(None));
            account_metas.extend(self.in_ata.to_account_metas(None));
            account_metas.extend(self.out_ata.to_account_metas(None));
            account_metas.extend(self.user.to_account_metas(None));
            account_metas.extend(self.user_out_ata.to_account_metas(None));
            account_metas.extend(self.init_user_out_ata.to_account_metas(None));
            account_metas.extend(self.intermediate_account.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.associated_token_program.to_account_metas(None));
            account_metas.extend(self.event_authority.to_account_metas(None));
            account_metas.extend(self.program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for EndAndClose<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.keeper, program_id)
                .map_err(|e| e.with_account_name("keeper"))?;
            anchor_lang::AccountsExit::exit(&self.dca, program_id)
                .map_err(|e| e.with_account_name("dca"))?;
            anchor_lang::AccountsExit::exit(&self.in_ata, program_id)
                .map_err(|e| e.with_account_name("in_ata"))?;
            anchor_lang::AccountsExit::exit(&self.out_ata, program_id)
                .map_err(|e| e.with_account_name("out_ata"))?;
            anchor_lang::AccountsExit::exit(&self.user, program_id)
                .map_err(|e| e.with_account_name("user"))?;
            anchor_lang::AccountsExit::exit(&self.user_out_ata, program_id)
                .map_err(|e| e.with_account_name("user_out_ata"))?;
            anchor_lang::AccountsExit::exit(&self.init_user_out_ata, program_id)
                .map_err(|e| e.with_account_name("init_user_out_ata"))?;
            anchor_lang::AccountsExit::exit(&self.intermediate_account, program_id)
                .map_err(|e| e.with_account_name("intermediate_account"))?;
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
    pub(crate) mod __client_accounts_end_and_close {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`EndAndClose`].
        pub struct EndAndClose {
            pub keeper: anchor_lang::solana_program::pubkey::Pubkey,
            pub dca: anchor_lang::solana_program::pubkey::Pubkey,
            pub input_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub output_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub in_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub user: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub init_user_out_ata: anchor_lang::solana_program::pubkey::Pubkey,
            pub intermediate_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub program: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for EndAndClose
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.keeper, writer)?;
                borsh::BorshSerialize::serialize(&self.dca, writer)?;
                borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.in_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.user_out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.init_user_out_ata, writer)?;
                borsh::BorshSerialize::serialize(&self.intermediate_account, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.associated_token_program,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.event_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for EndAndClose {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.keeper,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.dca,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.output_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.in_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.init_user_out_ata,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.intermediate_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.event_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.program,
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
    pub(crate) mod __cpi_client_accounts_end_and_close {
        use super::*;
        /// Generated CPI struct of the accounts for [`EndAndClose`].
        pub struct EndAndClose<'info> {
            pub keeper: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dca: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub input_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub output_mint: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub in_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub out_ata: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user_out_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub init_user_out_ata: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub intermediate_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub event_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for EndAndClose<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.keeper),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.dca),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.output_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.in_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.init_user_out_ata),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.intermediate_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.event_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for EndAndClose<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.keeper));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.dca));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.input_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.output_mint),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.in_ata));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.out_ata),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_out_ata),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.init_user_out_ata,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.intermediate_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.system_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.associated_token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.event_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                    );
                account_infos
            }
        }
    }
}
pub mod events {
    //! Structs of events generated by program.
    use super::*;
    pub struct CollectedFee {
        pub user_key: Pubkey,
        pub dca_key: Pubkey,
        pub mint: Pubkey,
        pub amount: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for CollectedFee {
        #[inline]
        fn default() -> CollectedFee {
            CollectedFee {
                user_key: ::core::default::Default::default(),
                dca_key: ::core::default::Default::default(),
                mint: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CollectedFee {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "CollectedFee",
                "user_key",
                &self.user_key,
                "dca_key",
                &self.dca_key,
                "mint",
                &self.mint,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CollectedFee {}
    impl borsh::ser::BorshSerialize for CollectedFee
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user_key, writer)?;
            borsh::BorshSerialize::serialize(&self.dca_key, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CollectedFee
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                user_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                dca_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CollectedFee {
        #[inline]
        fn clone(&self) -> CollectedFee {
            CollectedFee {
                user_key: ::core::clone::Clone::clone(&self.user_key),
                dca_key: ::core::clone::Clone::clone(&self.dca_key),
                mint: ::core::clone::Clone::clone(&self.mint),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl anchor_lang::Discriminator for CollectedFee {
        const DISCRIMINATOR: [u8; 8] = [42, 136, 216, 116, 181, 209, 109, 181];
        fn discriminator() -> [u8; 8] {
            self::DISCRIMINATOR
        }
    }
    pub struct Filled {
        pub user_key: Pubkey,
        pub dca_key: Pubkey,
        pub input_mint: Pubkey,
        pub output_mint: Pubkey,
        pub in_amount: u64,
        pub out_amount: u64,
        pub fee_mint: Pubkey,
        pub fee: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for Filled {
        #[inline]
        fn default() -> Filled {
            Filled {
                user_key: ::core::default::Default::default(),
                dca_key: ::core::default::Default::default(),
                input_mint: ::core::default::Default::default(),
                output_mint: ::core::default::Default::default(),
                in_amount: ::core::default::Default::default(),
                out_amount: ::core::default::Default::default(),
                fee_mint: ::core::default::Default::default(),
                fee: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Filled {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "user_key",
                "dca_key",
                "input_mint",
                "output_mint",
                "in_amount",
                "out_amount",
                "fee_mint",
                "fee",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.user_key,
                &self.dca_key,
                &self.input_mint,
                &self.output_mint,
                &self.in_amount,
                &self.out_amount,
                &self.fee_mint,
                &&self.fee,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Filled",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Filled {}
    impl borsh::ser::BorshSerialize for Filled
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user_key, writer)?;
            borsh::BorshSerialize::serialize(&self.dca_key, writer)?;
            borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.in_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.out_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.fee_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.fee, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Filled
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                user_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                dca_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                input_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                output_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                out_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Filled {
        #[inline]
        fn clone(&self) -> Filled {
            Filled {
                user_key: ::core::clone::Clone::clone(&self.user_key),
                dca_key: ::core::clone::Clone::clone(&self.dca_key),
                input_mint: ::core::clone::Clone::clone(&self.input_mint),
                output_mint: ::core::clone::Clone::clone(&self.output_mint),
                in_amount: ::core::clone::Clone::clone(&self.in_amount),
                out_amount: ::core::clone::Clone::clone(&self.out_amount),
                fee_mint: ::core::clone::Clone::clone(&self.fee_mint),
                fee: ::core::clone::Clone::clone(&self.fee),
            }
        }
    }
    impl anchor_lang::Discriminator for Filled {
        const DISCRIMINATOR: [u8; 8] = [134, 4, 17, 63, 221, 45, 177, 173];
        fn discriminator() -> [u8; 8] {
            self::DISCRIMINATOR
        }
    }
    pub struct Opened {
        pub user_key: Pubkey,
        pub dca_key: Pubkey,
        pub in_deposited: u64,
        pub input_mint: Pubkey,
        pub output_mint: Pubkey,
        pub cycle_frequency: i64,
        pub in_amount_per_cycle: u64,
        pub created_at: i64,
    }
    #[automatically_derived]
    impl ::core::default::Default for Opened {
        #[inline]
        fn default() -> Opened {
            Opened {
                user_key: ::core::default::Default::default(),
                dca_key: ::core::default::Default::default(),
                in_deposited: ::core::default::Default::default(),
                input_mint: ::core::default::Default::default(),
                output_mint: ::core::default::Default::default(),
                cycle_frequency: ::core::default::Default::default(),
                in_amount_per_cycle: ::core::default::Default::default(),
                created_at: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Opened {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "user_key",
                "dca_key",
                "in_deposited",
                "input_mint",
                "output_mint",
                "cycle_frequency",
                "in_amount_per_cycle",
                "created_at",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.user_key,
                &self.dca_key,
                &self.in_deposited,
                &self.input_mint,
                &self.output_mint,
                &self.cycle_frequency,
                &self.in_amount_per_cycle,
                &&self.created_at,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Opened",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Opened {}
    impl borsh::ser::BorshSerialize for Opened
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user_key, writer)?;
            borsh::BorshSerialize::serialize(&self.dca_key, writer)?;
            borsh::BorshSerialize::serialize(&self.in_deposited, writer)?;
            borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.cycle_frequency, writer)?;
            borsh::BorshSerialize::serialize(&self.in_amount_per_cycle, writer)?;
            borsh::BorshSerialize::serialize(&self.created_at, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Opened
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                user_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                dca_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_deposited: borsh::BorshDeserialize::deserialize_reader(reader)?,
                input_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                output_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                cycle_frequency: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_amount_per_cycle: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                created_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Opened {
        #[inline]
        fn clone(&self) -> Opened {
            Opened {
                user_key: ::core::clone::Clone::clone(&self.user_key),
                dca_key: ::core::clone::Clone::clone(&self.dca_key),
                in_deposited: ::core::clone::Clone::clone(&self.in_deposited),
                input_mint: ::core::clone::Clone::clone(&self.input_mint),
                output_mint: ::core::clone::Clone::clone(&self.output_mint),
                cycle_frequency: ::core::clone::Clone::clone(&self.cycle_frequency),
                in_amount_per_cycle: ::core::clone::Clone::clone(
                    &self.in_amount_per_cycle,
                ),
                created_at: ::core::clone::Clone::clone(&self.created_at),
            }
        }
    }
    impl anchor_lang::Discriminator for Opened {
        const DISCRIMINATOR: [u8; 8] = [166, 172, 97, 9, 77, 76, 189, 109];
        fn discriminator() -> [u8; 8] {
            self::DISCRIMINATOR
        }
    }
    pub struct Closed {
        pub user_key: Pubkey,
        pub dca_key: Pubkey,
        pub in_deposited: u64,
        pub input_mint: Pubkey,
        pub output_mint: Pubkey,
        pub cycle_frequency: i64,
        pub in_amount_per_cycle: u64,
        pub created_at: i64,
        pub total_in_withdrawn: u64,
        pub total_out_withdrawn: u64,
        pub unfilled_amount: u64,
        pub user_closed: bool,
    }
    #[automatically_derived]
    impl ::core::default::Default for Closed {
        #[inline]
        fn default() -> Closed {
            Closed {
                user_key: ::core::default::Default::default(),
                dca_key: ::core::default::Default::default(),
                in_deposited: ::core::default::Default::default(),
                input_mint: ::core::default::Default::default(),
                output_mint: ::core::default::Default::default(),
                cycle_frequency: ::core::default::Default::default(),
                in_amount_per_cycle: ::core::default::Default::default(),
                created_at: ::core::default::Default::default(),
                total_in_withdrawn: ::core::default::Default::default(),
                total_out_withdrawn: ::core::default::Default::default(),
                unfilled_amount: ::core::default::Default::default(),
                user_closed: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Closed {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "user_key",
                "dca_key",
                "in_deposited",
                "input_mint",
                "output_mint",
                "cycle_frequency",
                "in_amount_per_cycle",
                "created_at",
                "total_in_withdrawn",
                "total_out_withdrawn",
                "unfilled_amount",
                "user_closed",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.user_key,
                &self.dca_key,
                &self.in_deposited,
                &self.input_mint,
                &self.output_mint,
                &self.cycle_frequency,
                &self.in_amount_per_cycle,
                &self.created_at,
                &self.total_in_withdrawn,
                &self.total_out_withdrawn,
                &self.unfilled_amount,
                &&self.user_closed,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Closed",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Closed {}
    impl borsh::ser::BorshSerialize for Closed
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        bool: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.user_key, writer)?;
            borsh::BorshSerialize::serialize(&self.dca_key, writer)?;
            borsh::BorshSerialize::serialize(&self.in_deposited, writer)?;
            borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.cycle_frequency, writer)?;
            borsh::BorshSerialize::serialize(&self.in_amount_per_cycle, writer)?;
            borsh::BorshSerialize::serialize(&self.created_at, writer)?;
            borsh::BorshSerialize::serialize(&self.total_in_withdrawn, writer)?;
            borsh::BorshSerialize::serialize(&self.total_out_withdrawn, writer)?;
            borsh::BorshSerialize::serialize(&self.unfilled_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.user_closed, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Closed
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                user_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                dca_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_deposited: borsh::BorshDeserialize::deserialize_reader(reader)?,
                input_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                output_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                cycle_frequency: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_amount_per_cycle: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                created_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                total_in_withdrawn: borsh::BorshDeserialize::deserialize_reader(reader)?,
                total_out_withdrawn: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                unfilled_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                user_closed: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Closed {
        #[inline]
        fn clone(&self) -> Closed {
            Closed {
                user_key: ::core::clone::Clone::clone(&self.user_key),
                dca_key: ::core::clone::Clone::clone(&self.dca_key),
                in_deposited: ::core::clone::Clone::clone(&self.in_deposited),
                input_mint: ::core::clone::Clone::clone(&self.input_mint),
                output_mint: ::core::clone::Clone::clone(&self.output_mint),
                cycle_frequency: ::core::clone::Clone::clone(&self.cycle_frequency),
                in_amount_per_cycle: ::core::clone::Clone::clone(
                    &self.in_amount_per_cycle,
                ),
                created_at: ::core::clone::Clone::clone(&self.created_at),
                total_in_withdrawn: ::core::clone::Clone::clone(
                    &self.total_in_withdrawn,
                ),
                total_out_withdrawn: ::core::clone::Clone::clone(
                    &self.total_out_withdrawn,
                ),
                unfilled_amount: ::core::clone::Clone::clone(&self.unfilled_amount),
                user_closed: ::core::clone::Clone::clone(&self.user_closed),
            }
        }
    }
    impl anchor_lang::Discriminator for Closed {
        const DISCRIMINATOR: [u8; 8] = [50, 31, 87, 155, 135, 220, 195, 239];
        fn discriminator() -> [u8; 8] {
            self::DISCRIMINATOR
        }
    }
    pub struct Withdraw {
        pub dca_key: Pubkey,
        pub in_amount: u64,
        pub out_amount: u64,
        pub user_withdraw: bool,
    }
    #[automatically_derived]
    impl ::core::default::Default for Withdraw {
        #[inline]
        fn default() -> Withdraw {
            Withdraw {
                dca_key: ::core::default::Default::default(),
                in_amount: ::core::default::Default::default(),
                out_amount: ::core::default::Default::default(),
                user_withdraw: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Withdraw {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Withdraw",
                "dca_key",
                &self.dca_key,
                "in_amount",
                &self.in_amount,
                "out_amount",
                &self.out_amount,
                "user_withdraw",
                &&self.user_withdraw,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Withdraw {}
    impl borsh::ser::BorshSerialize for Withdraw
    where
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        bool: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.dca_key, writer)?;
            borsh::BorshSerialize::serialize(&self.in_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.out_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.user_withdraw, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Withdraw
    where
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                dca_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                in_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                out_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                user_withdraw: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Withdraw {
        #[inline]
        fn clone(&self) -> Withdraw {
            Withdraw {
                dca_key: ::core::clone::Clone::clone(&self.dca_key),
                in_amount: ::core::clone::Clone::clone(&self.in_amount),
                out_amount: ::core::clone::Clone::clone(&self.out_amount),
                user_withdraw: ::core::clone::Clone::clone(&self.user_withdraw),
            }
        }
    }
    impl anchor_lang::Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [192, 241, 201, 217, 70, 150, 90, 247];
        fn discriminator() -> [u8; 8] {
            self::DISCRIMINATOR
        }
    }
    pub struct Deposit {
        pub dca_key: Pubkey,
        pub amount: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for Deposit {
        #[inline]
        fn default() -> Deposit {
            Deposit {
                dca_key: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Deposit {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Deposit",
                "dca_key",
                &self.dca_key,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Deposit {}
    impl borsh::ser::BorshSerialize for Deposit
    where
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.dca_key, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Deposit
    where
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                dca_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Deposit {
        #[inline]
        fn clone(&self) -> Deposit {
            Deposit {
                dca_key: ::core::clone::Clone::clone(&self.dca_key),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl anchor_lang::Discriminator for Deposit {
        const DISCRIMINATOR: [u8; 8] = [62, 205, 242, 175, 244, 169, 136, 52];
        fn discriminator() -> [u8; 8] {
            self::DISCRIMINATOR
        }
    }
}
use ix_accounts::*;
pub use state::*;
pub use typedefs::*;
use self::dca::*;
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one categorie for now.
///
/// Global methods - regular methods inside of the `#[program]`.
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
    try_entry(program_id, accounts, data)
        .map_err(|e| {
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
    pub struct Dca;
    #[automatically_derived]
    impl ::core::clone::Clone for Dca {
        #[inline]
        fn clone(&self) -> Dca {
            Dca
        }
    }
    impl anchor_lang::Id for Dca {
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
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one type. "global" for a
/// regular instruction.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::OpenDca::DISCRIMINATOR => {
            __private::__global::open_dca(program_id, accounts, ix_data)
        }
        instruction::OpenDcaV2::DISCRIMINATOR => {
            __private::__global::open_dca_v2(program_id, accounts, ix_data)
        }
        instruction::CloseDca::DISCRIMINATOR => {
            __private::__global::close_dca(program_id, accounts, ix_data)
        }
        instruction::Withdraw::DISCRIMINATOR => {
            __private::__global::withdraw(program_id, accounts, ix_data)
        }
        instruction::Deposit::DISCRIMINATOR => {
            __private::__global::deposit(program_id, accounts, ix_data)
        }
        instruction::WithdrawFees::DISCRIMINATOR => {
            __private::__global::withdraw_fees(program_id, accounts, ix_data)
        }
        instruction::InitiateFlashFill::DISCRIMINATOR => {
            __private::__global::initiate_flash_fill(program_id, accounts, ix_data)
        }
        instruction::FulfillFlashFill::DISCRIMINATOR => {
            __private::__global::fulfill_flash_fill(program_id, accounts, ix_data)
        }
        instruction::Transfer::DISCRIMINATOR => {
            __private::__global::transfer(program_id, accounts, ix_data)
        }
        instruction::EndAndClose::DISCRIMINATOR => {
            __private::__global::end_and_close(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {
            __private::__idl::__idl_dispatch(program_id, accounts, &ix_data)
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
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
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlResizeAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "IdlAccount",
                    "authority",
                    &self.authority,
                    "data_len",
                    &&self.data_len,
                )
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount
        where
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123, 158]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "src/lib.rs",
                                        line: 1u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdlAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] = [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer = Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                    .expect("Seed is always valid")
            }
            pub fn seed() -> &'static str {
                "anchor:idl"
            }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let from: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("base"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("program"))?;
                if !from.is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("from"),
                    );
                }
                if !to.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[],
                    &__program_id,
                );
                __bumps.insert("base".to_string(), __bump);
                if base.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("base")
                            .with_pubkeys((base.key(), __pda_address)),
                    );
                }
                if !program.to_account_info().executable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintExecutable,
                            )
                            .with_account_name("program"),
                    );
                }
                Ok(IdlCreateAccounts {
                    from,
                    to,
                    base,
                    system_program,
                    program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
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
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts {
                pub from: anchor_lang::solana_program::pubkey::Pubkey,
                pub to: anchor_lang::solana_program::pubkey::Pubkey,
                pub base: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts
            where
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
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.from,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.base,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.program,
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
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.from),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.base),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.from),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.base),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !idl.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
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
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlAccounts`].
            pub struct IdlAccounts {
                pub idl: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
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
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlAccounts`].
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !idl.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !authority.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlResizeAccount {
                    idl,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
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
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount {
                pub idl: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
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
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = {
                    let mut __data: &[u8] = &buffer.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintZero,
                                )
                                .with_account_name("buffer"),
                        );
                    }
                    match anchor_lang::accounts::account::Account::try_from_unchecked(
                        &buffer,
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(e.with_account_name("buffer")),
                    }
                };
                if !buffer.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        buffer.to_account_info().lamports(),
                        buffer.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlCreateBuffer {
                    buffer,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
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
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer {
                pub buffer: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
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
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority = = idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !buffer.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(buffer.authority == idl.authority) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !idl.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlSetBuffer {
                    buffer,
                    idl,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
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
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer {
                pub buffer: anchor_lang::solana_program::pubkey::Pubkey,
                pub idl: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
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
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let account: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("account"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                if !account.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("account"),
                    );
                }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("account")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                {
                    if account.key() == sol_destination.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("account"),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !sol_destination.to_account_info().is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("sol_destination"),
                    );
                }
                Ok(IdlCloseAccount {
                    account,
                    authority,
                    sol_destination,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(
                            &self.account,
                            sol_destination.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination, program_id)
                    .map_err(|e| e.with_account_name("sol_destination"))?;
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
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount {
                pub account: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub sol_destination: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.sol_destination,
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
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.sol_destination),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.account),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.sol_destination,
                            ),
                        );
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self) -> Ref<'info, [u8]>;
            fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
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
                    accounts.system_program.to_account_info().clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(
            program_id: &Pubkey,
            accounts: &mut IdlResizeAccount,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
            }
            let new_account_space = accounts
                .idl
                .to_account_info()
                .data_len()
                .checked_add(
                    std::cmp::min(
                        data_len
                            .checked_sub(accounts.idl.to_account_info().data_len())
                            .expect(
                                "data_len should always be >= the current account space",
                            ),
                        10_000,
                    ),
                )
                .unwrap();
            if new_account_space > accounts.idl.to_account_info().data_len() {
                let sysvar_rent = Rent::get()?;
                let new_rent_minimum = sysvar_rent.minimum_balance(new_account_space);
                anchor_lang::system_program::transfer(
                    anchor_lang::context::CpiContext::new(
                        accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: accounts.authority.to_account_info(),
                            to: accounts.idl.to_account_info().clone(),
                        },
                    ),
                    new_rent_minimum
                        .checked_sub(accounts.idl.to_account_info().lamports())
                        .unwrap(),
                )?;
                accounts.idl.to_account_info().realloc(new_account_space, false)?;
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.idl.data_len)
                .unwrap();
            let new_len: usize = prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts
                .idl
                .data_len = accounts
                .idl
                .data_len
                .checked_add(
                    ::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap(),
                )
                .unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireEqViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireEqViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "src/lib.rs",
                                    line: 1u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((idl_expansion.len(), idl_data.len())),
                );
            }
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.buffer.data_len)
                .unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireGteViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireGteViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "src/lib.rs",
                                    line: 1u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((target.len(), buffer_len)),
                );
            }
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn open_dca(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: OpenDca");
            let ix = instruction::OpenDca::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::OpenDca {
                _application_idx,
                _in_amount,
                _in_amount_per_cycle,
                _cycle_frequency,
                _min_price,
                _max_price,
                _start_at,
                _close_wsol_in_ata,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = OpenDca::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::open_dca(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _application_idx,
                _in_amount,
                _in_amount_per_cycle,
                _cycle_frequency,
                _min_price,
                _max_price,
                _start_at,
                _close_wsol_in_ata,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn open_dca_v2(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: OpenDcaV2");
            let ix = instruction::OpenDcaV2::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::OpenDcaV2 {
                _application_idx,
                _in_amount,
                _in_amount_per_cycle,
                _cycle_frequency,
                _min_price,
                _max_price,
                _start_at,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = OpenDcaV2::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::open_dca_v2(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _application_idx,
                _in_amount,
                _in_amount_per_cycle,
                _cycle_frequency,
                _min_price,
                _max_price,
                _start_at,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn close_dca(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CloseDca");
            let ix = instruction::CloseDca::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CloseDca = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CloseDca::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::close_dca(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Withdraw");
            let ix = instruction::Withdraw::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Withdraw { _withdraw_params } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Withdraw::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::withdraw(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _withdraw_params,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn deposit(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Deposit");
            let ix = instruction::Deposit::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Deposit { _deposit_in } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Deposit::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::deposit(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _deposit_in,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw_fees(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawFees");
            let ix = instruction::WithdrawFees::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::WithdrawFees { _amount } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = WithdrawFees::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::withdraw_fees(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _amount,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initiate_flash_fill(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitiateFlashFill");
            let ix = instruction::InitiateFlashFill::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitiateFlashFill = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitiateFlashFill::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::initiate_flash_fill(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn fulfill_flash_fill(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: FulfillFlashFill");
            let ix = instruction::FulfillFlashFill::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::FulfillFlashFill { _repay_amount } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = FulfillFlashFill::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::fulfill_flash_fill(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _repay_amount,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn transfer(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Transfer");
            let ix = instruction::Transfer::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Transfer = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Transfer::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::transfer(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn end_and_close(
            __program_id: &Pubkey,
            __accounts: &[AccountInfo],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: EndAndClose");
            let ix = instruction::EndAndClose::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::EndAndClose = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = EndAndClose::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = dca::end_and_close(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod dca {
    //! Anchor CPI crate generated from dca v0.1.0 using [anchor-gen](https://crates.io/crates/anchor-gen) v0.3.1.
    use super::*;
    pub fn open_dca(
        _ctx: Context<OpenDca>,
        _application_idx: u64,
        _in_amount: u64,
        _in_amount_per_cycle: u64,
        _cycle_frequency: i64,
        _min_price: Option<u64>,
        _max_price: Option<u64>,
        _start_at: Option<i64>,
        _close_wsol_in_ata: Option<bool>,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn open_dca_v2(
        _ctx: Context<OpenDcaV2>,
        _application_idx: u64,
        _in_amount: u64,
        _in_amount_per_cycle: u64,
        _cycle_frequency: i64,
        _min_price: Option<u64>,
        _max_price: Option<u64>,
        _start_at: Option<i64>,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn close_dca(_ctx: Context<CloseDca>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn withdraw(
        _ctx: Context<Withdraw>,
        _withdraw_params: WithdrawParams,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn deposit(_ctx: Context<Deposit>, _deposit_in: u64) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn withdraw_fees(_ctx: Context<WithdrawFees>, _amount: u64) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn initiate_flash_fill(_ctx: Context<InitiateFlashFill>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn fulfill_flash_fill(
        _ctx: Context<FulfillFlashFill>,
        _repay_amount: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn transfer(_ctx: Context<Transfer>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn end_and_close(_ctx: Context<EndAndClose>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
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
    /// Instruction.
    pub struct OpenDca {
        pub _application_idx: u64,
        pub _in_amount: u64,
        pub _in_amount_per_cycle: u64,
        pub _cycle_frequency: i64,
        pub _min_price: Option<u64>,
        pub _max_price: Option<u64>,
        pub _start_at: Option<i64>,
        pub _close_wsol_in_ata: Option<bool>,
    }
    impl borsh::ser::BorshSerialize for OpenDca
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        Option<u64>: borsh::ser::BorshSerialize,
        Option<u64>: borsh::ser::BorshSerialize,
        Option<i64>: borsh::ser::BorshSerialize,
        Option<bool>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._application_idx, writer)?;
            borsh::BorshSerialize::serialize(&self._in_amount, writer)?;
            borsh::BorshSerialize::serialize(&self._in_amount_per_cycle, writer)?;
            borsh::BorshSerialize::serialize(&self._cycle_frequency, writer)?;
            borsh::BorshSerialize::serialize(&self._min_price, writer)?;
            borsh::BorshSerialize::serialize(&self._max_price, writer)?;
            borsh::BorshSerialize::serialize(&self._start_at, writer)?;
            borsh::BorshSerialize::serialize(&self._close_wsol_in_ata, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for OpenDca
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        Option<u64>: borsh::BorshDeserialize,
        Option<u64>: borsh::BorshDeserialize,
        Option<i64>: borsh::BorshDeserialize,
        Option<bool>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _application_idx: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _in_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _in_amount_per_cycle: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                _cycle_frequency: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _min_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _max_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _start_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _close_wsol_in_ata: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for OpenDca {
        const DISCRIMINATOR: [u8; 8] = [36, 65, 185, 54, 1, 210, 100, 163];
    }
    impl anchor_lang::InstructionData for OpenDca {}
    impl anchor_lang::Owner for OpenDca {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct OpenDcaV2 {
        pub _application_idx: u64,
        pub _in_amount: u64,
        pub _in_amount_per_cycle: u64,
        pub _cycle_frequency: i64,
        pub _min_price: Option<u64>,
        pub _max_price: Option<u64>,
        pub _start_at: Option<i64>,
    }
    impl borsh::ser::BorshSerialize for OpenDcaV2
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        Option<u64>: borsh::ser::BorshSerialize,
        Option<u64>: borsh::ser::BorshSerialize,
        Option<i64>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._application_idx, writer)?;
            borsh::BorshSerialize::serialize(&self._in_amount, writer)?;
            borsh::BorshSerialize::serialize(&self._in_amount_per_cycle, writer)?;
            borsh::BorshSerialize::serialize(&self._cycle_frequency, writer)?;
            borsh::BorshSerialize::serialize(&self._min_price, writer)?;
            borsh::BorshSerialize::serialize(&self._max_price, writer)?;
            borsh::BorshSerialize::serialize(&self._start_at, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for OpenDcaV2
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        Option<u64>: borsh::BorshDeserialize,
        Option<u64>: borsh::BorshDeserialize,
        Option<i64>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _application_idx: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _in_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _in_amount_per_cycle: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                _cycle_frequency: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _min_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _max_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _start_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for OpenDcaV2 {
        const DISCRIMINATOR: [u8; 8] = [142, 119, 43, 109, 162, 52, 11, 177];
    }
    impl anchor_lang::InstructionData for OpenDcaV2 {}
    impl anchor_lang::Owner for OpenDcaV2 {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CloseDca;
    impl borsh::ser::BorshSerialize for CloseDca {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CloseDca {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for CloseDca {
        const DISCRIMINATOR: [u8; 8] = [22, 7, 33, 98, 168, 183, 34, 243];
    }
    impl anchor_lang::InstructionData for CloseDca {}
    impl anchor_lang::Owner for CloseDca {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Withdraw {
        pub _withdraw_params: WithdrawParams,
    }
    impl borsh::ser::BorshSerialize for Withdraw
    where
        WithdrawParams: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._withdraw_params, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Withdraw
    where
        WithdrawParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _withdraw_params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
    }
    impl anchor_lang::InstructionData for Withdraw {}
    impl anchor_lang::Owner for Withdraw {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Deposit {
        pub _deposit_in: u64,
    }
    impl borsh::ser::BorshSerialize for Deposit
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._deposit_in, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Deposit
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _deposit_in: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for Deposit {
        const DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
    }
    impl anchor_lang::InstructionData for Deposit {}
    impl anchor_lang::Owner for Deposit {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct WithdrawFees {
        pub _amount: u64,
    }
    impl borsh::ser::BorshSerialize for WithdrawFees
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawFees
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for WithdrawFees {
        const DISCRIMINATOR: [u8; 8] = [198, 212, 171, 109, 144, 215, 174, 89];
    }
    impl anchor_lang::InstructionData for WithdrawFees {}
    impl anchor_lang::Owner for WithdrawFees {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitiateFlashFill;
    impl borsh::ser::BorshSerialize for InitiateFlashFill {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitiateFlashFill {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for InitiateFlashFill {
        const DISCRIMINATOR: [u8; 8] = [143, 205, 3, 191, 162, 215, 245, 49];
    }
    impl anchor_lang::InstructionData for InitiateFlashFill {}
    impl anchor_lang::Owner for InitiateFlashFill {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct FulfillFlashFill {
        pub _repay_amount: u64,
    }
    impl borsh::ser::BorshSerialize for FulfillFlashFill
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._repay_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FulfillFlashFill
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _repay_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for FulfillFlashFill {
        const DISCRIMINATOR: [u8; 8] = [115, 64, 226, 78, 33, 211, 105, 162];
    }
    impl anchor_lang::InstructionData for FulfillFlashFill {}
    impl anchor_lang::Owner for FulfillFlashFill {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Transfer;
    impl borsh::ser::BorshSerialize for Transfer {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Transfer {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for Transfer {
        const DISCRIMINATOR: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];
    }
    impl anchor_lang::InstructionData for Transfer {}
    impl anchor_lang::Owner for Transfer {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct EndAndClose;
    impl borsh::ser::BorshSerialize for EndAndClose {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for EndAndClose {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for EndAndClose {
        const DISCRIMINATOR: [u8; 8] = [83, 125, 166, 69, 247, 252, 103, 133];
    }
    impl anchor_lang::InstructionData for EndAndClose {}
    impl anchor_lang::Owner for EndAndClose {
        fn owner() -> Pubkey {
            ID
        }
    }
}
#[cfg(feature = "cpi")]
pub mod cpi {
    use super::*;
    use std::marker::PhantomData;
    pub struct Return<T> {
        phantom: std::marker::PhantomData<T>,
    }
    impl<T: AnchorDeserialize> Return<T> {
        pub fn get(&self) -> T {
            let (_key, data) = anchor_lang::solana_program::program::get_return_data()
                .unwrap();
            T::try_from_slice(&data).unwrap()
        }
    }
    pub fn open_dca<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::OpenDca<'info>,
        >,
        _application_idx: u64,
        _in_amount: u64,
        _in_amount_per_cycle: u64,
        _cycle_frequency: i64,
        _min_price: Option<u64>,
        _max_price: Option<u64>,
        _start_at: Option<i64>,
        _close_wsol_in_ata: Option<bool>,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::OpenDca {
                _application_idx,
                _in_amount,
                _in_amount_per_cycle,
                _cycle_frequency,
                _min_price,
                _max_price,
                _start_at,
                _close_wsol_in_ata,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [36, 65, 185, 54, 1, 210, 100, 163].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn open_dca_v2<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::OpenDcaV2<'info>,
        >,
        _application_idx: u64,
        _in_amount: u64,
        _in_amount_per_cycle: u64,
        _cycle_frequency: i64,
        _min_price: Option<u64>,
        _max_price: Option<u64>,
        _start_at: Option<i64>,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::OpenDcaV2 {
                _application_idx,
                _in_amount,
                _in_amount_per_cycle,
                _cycle_frequency,
                _min_price,
                _max_price,
                _start_at,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [142, 119, 43, 109, 162, 52, 11, 177].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn close_dca<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::CloseDca<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::CloseDca;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [22, 7, 33, 98, 168, 183, 34, 243].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::Withdraw<'info>,
        >,
        _withdraw_params: WithdrawParams,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::Withdraw {
                _withdraw_params,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [183, 18, 70, 156, 148, 109, 161, 34].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::Deposit<'info>,
        >,
        _deposit_in: u64,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::Deposit {
                _deposit_in,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [242, 35, 198, 137, 82, 225, 242, 182].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn withdraw_fees<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::WithdrawFees<'info>,
        >,
        _amount: u64,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::WithdrawFees {
                _amount,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [198, 212, 171, 109, 144, 215, 174, 89].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn initiate_flash_fill<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::InitiateFlashFill<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::InitiateFlashFill;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [143, 205, 3, 191, 162, 215, 245, 49].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn fulfill_flash_fill<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::FulfillFlashFill<'info>,
        >,
        _repay_amount: u64,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::FulfillFlashFill {
                _repay_amount,
            };
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [115, 64, 226, 78, 33, 211, 105, 162].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn transfer<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::Transfer<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::Transfer;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [163, 52, 200, 231, 140, 3, 69, 186].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    pub fn end_and_close<'a, 'b, 'c, 'info>(
        ctx: anchor_lang::context::CpiContext<
            'a,
            'b,
            'c,
            'info,
            crate::cpi::accounts::EndAndClose<'info>,
        >,
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instruction::EndAndClose;
            let mut ix_data = AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [83, 125, 166, 69, 247, 252, 103, 133].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: crate::ID,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &acc_infos,
                ctx.signer_seeds,
            )
            .map_or_else(|e| Err(Into::into(e)), |_| { Ok(()) })
    }
    /// An Anchor generated module, providing a set of structs
    /// mirroring the structs deriving `Accounts`, where each field is
    /// an `AccountInfo`. This is useful for CPI.
    pub mod accounts {
        pub use crate::__cpi_client_accounts_open_dca::*;
        pub use crate::__cpi_client_accounts_open_dca_v2::*;
        pub use crate::__cpi_client_accounts_deposit::*;
        pub use crate::__cpi_client_accounts_fulfill_flash_fill::*;
        pub use crate::__cpi_client_accounts_withdraw_fees::*;
        pub use crate::__cpi_client_accounts_transfer::*;
        pub use crate::__cpi_client_accounts_end_and_close::*;
        pub use crate::__cpi_client_accounts_close_dca::*;
        pub use crate::__cpi_client_accounts_initiate_flash_fill::*;
        pub use crate::__cpi_client_accounts_withdraw::*;
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_initiate_flash_fill::*;
    pub use crate::__client_accounts_open_dca_v2::*;
    pub use crate::__client_accounts_withdraw_fees::*;
    pub use crate::__client_accounts_deposit::*;
    pub use crate::__client_accounts_transfer::*;
    pub use crate::__client_accounts_withdraw::*;
    pub use crate::__client_accounts_close_dca::*;
    pub use crate::__client_accounts_open_dca::*;
    pub use crate::__client_accounts_fulfill_flash_fill::*;
    pub use crate::__client_accounts_end_and_close::*;
}
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    181u8,
    39u8,
    212u8,
    93u8,
    100u8,
    14u8,
    62u8,
    193u8,
    95u8,
    97u8,
    93u8,
    91u8,
    14u8,
    35u8,
    165u8,
    74u8,
    150u8,
    184u8,
    63u8,
    11u8,
    130u8,
    11u8,
    247u8,
    6u8,
    53u8,
    242u8,
    67u8,
    74u8,
    198u8,
    64u8,
    204u8,
    4u8,
]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
