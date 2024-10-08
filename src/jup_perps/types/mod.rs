//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

  pub(crate) mod r#add_custody_params;
  pub(crate) mod r#add_liquidity2_params;
  pub(crate) mod r#add_liquidity_params;
  pub(crate) mod r#add_pool_params;
  pub(crate) mod r#amount_and_fee;
  pub(crate) mod r#assets;
  pub(crate) mod r#close_position_request_params;
  pub(crate) mod r#create_decrease_position_market_request_params;
  pub(crate) mod r#create_decrease_position_request2_params;
  pub(crate) mod r#create_decrease_position_request_params;
  pub(crate) mod r#create_increase_position_market_request_params;
  pub(crate) mod r#create_increase_position_request_params;
  pub(crate) mod r#create_token_metadata_params;
  pub(crate) mod r#decrease_position2_params;
  pub(crate) mod r#decrease_position3_params;
  pub(crate) mod r#decrease_position4_params;
  pub(crate) mod r#decrease_position_info;
  pub(crate) mod r#decrease_position_post_swap_params;
  pub(crate) mod r#fees;
  pub(crate) mod r#funding_rate_state;
  pub(crate) mod r#get_add_liquidity_amount_and_fee2_params;
  pub(crate) mod r#get_add_liquidity_amount_and_fee_params;
  pub(crate) mod r#get_assets_under_management2_params;
  pub(crate) mod r#get_assets_under_management_params;
  pub(crate) mod r#get_decrease_position_params;
  pub(crate) mod r#get_exact_out_swap_amount_and_fees_params;
  pub(crate) mod r#get_increase_position_params;
  pub(crate) mod r#get_liquidation_state_params;
  pub(crate) mod r#get_pnl_and_fee_params;
  pub(crate) mod r#get_remove_liquidity_amount_and_fee2_params;
  pub(crate) mod r#get_remove_liquidity_amount_and_fee_params;
  pub(crate) mod r#get_swap_amount_and_fees_params;
  pub(crate) mod r#increase_position2_params;
  pub(crate) mod r#increase_position4_params;
  pub(crate) mod r#increase_position_info;
  pub(crate) mod r#increase_position_pre_swap_params;
  pub(crate) mod r#init_params;
  pub(crate) mod r#instant_create_tpsl_params;
  pub(crate) mod r#instant_update_tpsl_params;
  pub(crate) mod r#limit;
  pub(crate) mod r#liquidate_full_position2_params;
  pub(crate) mod r#liquidate_full_position4_params;
  pub(crate) mod r#oracle_params;
  pub(crate) mod r#oracle_price;
  pub(crate) mod r#oracle_price_info;
  pub(crate) mod r#oracle_type;
  pub(crate) mod r#permissions;
  pub(crate) mod r#pnl_and_fee;
  pub(crate) mod r#pool_apr;
  pub(crate) mod r#price_calc_mode;
  pub(crate) mod r#price_stale_tolerance;
  pub(crate) mod r#pricing_params;
  pub(crate) mod r#refresh_assets_under_management_params;
  pub(crate) mod r#remove_liquidity2_params;
  pub(crate) mod r#remove_liquidity_params;
  pub(crate) mod r#request_change;
  pub(crate) mod r#request_type;
  pub(crate) mod r#set_custody_config_params;
  pub(crate) mod r#set_custody_global_limit_params;
  pub(crate) mod r#set_perpetuals_config_params;
  pub(crate) mod r#set_pool_config_params;
  pub(crate) mod r#set_test_oracle_price_params;
  pub(crate) mod r#set_test_time_params;
  pub(crate) mod r#side;
  pub(crate) mod r#swap2_params;
  pub(crate) mod r#swap_amount_and_fees;
  pub(crate) mod r#swap_exact_out_params;
  pub(crate) mod r#swap_params;
  pub(crate) mod r#test_init_params;
  pub(crate) mod r#transfer_admin_params;
  pub(crate) mod r#update_decrease_position_request2_params;
  pub(crate) mod r#update_decrease_position_request_params;
  pub(crate) mod r#update_increase_position_request_params;
  pub(crate) mod r#withdraw_fees2_params;
  pub(crate) mod r#withdraw_fees_params;

  pub use self::r#add_custody_params::*;
  pub use self::r#add_liquidity2_params::*;
  pub use self::r#add_liquidity_params::*;
  pub use self::r#add_pool_params::*;
  pub use self::r#amount_and_fee::*;
  pub use self::r#assets::*;
  pub use self::r#close_position_request_params::*;
  pub use self::r#create_decrease_position_market_request_params::*;
  pub use self::r#create_decrease_position_request2_params::*;
  pub use self::r#create_decrease_position_request_params::*;
  pub use self::r#create_increase_position_market_request_params::*;
  pub use self::r#create_increase_position_request_params::*;
  pub use self::r#create_token_metadata_params::*;
  pub use self::r#decrease_position2_params::*;
  pub use self::r#decrease_position3_params::*;
  pub use self::r#decrease_position4_params::*;
  pub use self::r#decrease_position_info::*;
  pub use self::r#decrease_position_post_swap_params::*;
  pub use self::r#fees::*;
  pub use self::r#funding_rate_state::*;
  pub use self::r#get_add_liquidity_amount_and_fee2_params::*;
  pub use self::r#get_add_liquidity_amount_and_fee_params::*;
  pub use self::r#get_assets_under_management2_params::*;
  pub use self::r#get_assets_under_management_params::*;
  pub use self::r#get_decrease_position_params::*;
  pub use self::r#get_exact_out_swap_amount_and_fees_params::*;
  pub use self::r#get_increase_position_params::*;
  pub use self::r#get_liquidation_state_params::*;
  pub use self::r#get_pnl_and_fee_params::*;
  pub use self::r#get_remove_liquidity_amount_and_fee2_params::*;
  pub use self::r#get_remove_liquidity_amount_and_fee_params::*;
  pub use self::r#get_swap_amount_and_fees_params::*;
  pub use self::r#increase_position2_params::*;
  pub use self::r#increase_position4_params::*;
  pub use self::r#increase_position_info::*;
  pub use self::r#increase_position_pre_swap_params::*;
  pub use self::r#init_params::*;
  pub use self::r#instant_create_tpsl_params::*;
  pub use self::r#instant_update_tpsl_params::*;
  pub use self::r#limit::*;
  pub use self::r#liquidate_full_position2_params::*;
  pub use self::r#liquidate_full_position4_params::*;
  pub use self::r#oracle_params::*;
  pub use self::r#oracle_price::*;
  pub use self::r#oracle_price_info::*;
  pub use self::r#oracle_type::*;
  pub use self::r#permissions::*;
  pub use self::r#pnl_and_fee::*;
  pub use self::r#pool_apr::*;
  pub use self::r#price_calc_mode::*;
  pub use self::r#price_stale_tolerance::*;
  pub use self::r#pricing_params::*;
  pub use self::r#refresh_assets_under_management_params::*;
  pub use self::r#remove_liquidity2_params::*;
  pub use self::r#remove_liquidity_params::*;
  pub use self::r#request_change::*;
  pub use self::r#request_type::*;
  pub use self::r#set_custody_config_params::*;
  pub use self::r#set_custody_global_limit_params::*;
  pub use self::r#set_perpetuals_config_params::*;
  pub use self::r#set_pool_config_params::*;
  pub use self::r#set_test_oracle_price_params::*;
  pub use self::r#set_test_time_params::*;
  pub use self::r#side::*;
  pub use self::r#swap2_params::*;
  pub use self::r#swap_amount_and_fees::*;
  pub use self::r#swap_exact_out_params::*;
  pub use self::r#swap_params::*;
  pub use self::r#test_init_params::*;
  pub use self::r#transfer_admin_params::*;
  pub use self::r#update_decrease_position_request2_params::*;
  pub use self::r#update_decrease_position_request_params::*;
  pub use self::r#update_increase_position_request_params::*;
  pub use self::r#withdraw_fees2_params::*;
  pub use self::r#withdraw_fees_params::*;

