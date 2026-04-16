use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::Com::*,
        UI::Shell::*,
    },
};

pub fn open_file_dialog() -> Option<String> {
    unsafe {
        if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
            return None;
        }

        let result: Result<String> = (|| {
            let dialog: IFileOpenDialog =
                CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL)?;

            dialog.SetTitle(w!("Unitypackageを選択"))?;

            dialog.Show(Some(HWND::default()))?;

            let item: IShellItem = dialog.GetResult()?;

            let path = item.GetDisplayName(SIGDN_FILESYSPATH)?;
            let path_string = path.to_string()?;
            CoTaskMemFree(Some(path.0 as _));

            Ok(path_string)
        })();

        CoUninitialize();

        result.ok()
    }
}