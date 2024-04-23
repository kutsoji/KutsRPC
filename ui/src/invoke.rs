#[macro_export]
macro_rules! invoke {
    ($cmd:expr, $ret_type:ty) => {
        tauri_sys::tauri::invoke::<_, $ret_type>($cmd, &()).await
    };
    ($cmd:expr) => {
        tauri_sys::tauri::invoke::<_, ()>($cmd, &()).await
    };
    ($cmd:expr, {$($key:ident: $type:ty = $value:expr),+ $(,)?}, $ret_type:ty) => {
        {
            #[derive(serde::Serialize)]
            struct Args {
                $($key: $type),+
            }
            let args = Args {
                $($key: $value),+
            };
            tauri_sys::tauri::invoke::<_, $ret_type>($cmd, &args).await
        }
    };
    ($cmd:expr, {$($key:ident: $type:ty = $value:expr),+ $(,)?}) => {
        {
            #[derive(serde::Serialize)]
            struct Args {
                $($key: $type),+
            }
            let args = Args {
                $($key: $value),+
            };
            tauri_sys::tauri::invoke::<_, ()>($cmd, &args).await
        }
    };
}
