pub mod sgv0000 {
    use native_functions::zera::smart_contracts;
    use native_functions::zera::types;
    use native_functions::zera::types::U256;
    use native_functions::zera::wasmedge_bindgen;

    const ANY_SEND_KEY: &str = "ANY_";
    const STAGE_KEY: &str = "STAGE_";
    const GOV_SGV: &str = "gov_$SGV+0000";
    const ZRA_CONTRACT: &str = "$ZRA+0000";
    const ANY_SEND_AMOUNT: &str = "55000000000000";
    const SEND_FUNDS_AMOUNT: &str = "106000000000000";
    const STAGE_SEND_AMOUNT: &str = "17000000000000";

    const RECIPIENT_WALLET: &str = "7er2pZn9tBLp6Tr8Xt8EwSmjhexfXKuE4AvDhtBRD9mq";
    const TREASURY_WALLET: &str = "4Yg2ZeYrzMjVBXvU2YWtuZ7CzWR9atnQCD35TQj1kKcH";
    const REDEMPTION_DATE: u64 = 1793426400;

    #[wasmedge_bindgen]
    pub fn init() {
        unsafe{
            let (authorized, rate) = smart_contracts::get_ace_data(ZRA_CONTRACT.to_string());
            let denomination = smart_contracts::contract_denomination(ZRA_CONTRACT.to_string());
            let one_dolla = types::string_to_u256("5000000000000000000".to_string()); //change to 5$
            let one_dolla_zera = (one_dolla * denomination) / rate;

            smart_contracts::hold(ZRA_CONTRACT.to_string(), one_dolla_zera.to_string());
        }
    }

    #[wasmedge_bindgen]
    pub fn stage_send() {
        unsafe {
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != GOV_SGV.to_string() {
                return;
            }

            let mut stage_str = smart_contracts::retrieve_state(STAGE_KEY.to_string());

            if(stage_str.is_empty()){
                stage_str = "0".to_string();
            }

            let stage: i32 = stage_str.parse().unwrap_or(0);
            
            let next_stage = stage + 1;
            
            if(next_stage > 3)
            {
                return;
            }

            smart_contracts::store_state(STAGE_KEY.to_string(), next_stage.to_string());

            smart_contracts::send(ZRA_CONTRACT.to_string(), STAGE_SEND_AMOUNT.to_string(), RECIPIENT_WALLET.to_string());
        }
    }

    #[wasmedge_bindgen]
    pub fn any_send(wallet_address: String, amount: String) {
        unsafe {
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != GOV_SGV.to_string() {
                return;
            }

            let mut amount_sent_str = smart_contracts::retrieve_state(ANY_SEND_KEY.to_string());

            if amount_sent_str.is_empty() {
                amount_sent_str = "0".to_string();
            }

            let amount_sent = types::string_to_u256(amount_sent_str);

            if !types::is_valid_u256(amount.clone()) {
                return;
            }

            let amount_u256 = types::string_to_u256(amount.clone());

            let any_amount = types::string_to_u256(ANY_SEND_AMOUNT.to_string());

            let total_amount = amount_sent + amount_u256;

            if total_amount > any_amount {
                return;
            }

            smart_contracts::send(ZRA_CONTRACT.to_string(), amount.clone(), wallet_address.clone());

            smart_contracts::store_state(ANY_SEND_KEY.to_string(), total_amount.to_string());
        }
    }

    #[wasmedge_bindgen]
    pub fn send_all() {
        unsafe {
            let last_block_time = smart_contracts::last_block_time();

            if last_block_time < REDEMPTION_DATE {
                return;
            }

            smart_contracts::send_all(TREASURY_WALLET.to_string());
        }
    }

}
