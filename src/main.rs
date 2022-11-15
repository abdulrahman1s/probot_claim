pub mod claim;
pub mod utils;
use claim::spawn_calim;
use utils::{get_tokens, get_chrome_extionsion_path};
#[tokio::main]
async fn main() {
    let chrome_extionsion_path = get_chrome_extionsion_path();
    let tokens = get_tokens();
    for token in tokens {
        spawn_calim(token.to_string(), chrome_extionsion_path.as_str());
    }
}
