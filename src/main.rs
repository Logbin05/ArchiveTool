use std::process::Command;
use chrono::Local;
use rfd::MessageDialog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_folder = rfd::FileDialog::new()
        .set_title("Выберите папку с логами/файлами")
        .pick_folder()
        .expect("Папка не выбрана")
        .to_string_lossy()
        .to_string();

    let output_dir = rfd::FileDialog::new()
        .set_title("Выберите папку для сохранения архива")
        .pick_folder()
        .expect("Папка не выбрана")
        .to_string_lossy()
        .to_string();

    let archive_base_name = rfd::FileDialog::new()
        .set_title("Введите имя архива (без расширения)")
        .set_file_name("")
        .save_file()
        .expect("Имя архива не введено")
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let now = Local::now();
    let date_str = now.format("%d.%m.%Y_%H-%M-%S").to_string();

    let archive_name = format!("{}_{}.exe", archive_base_name, date_str);
    let archive_path = format!(r"{}\{}", output_dir, archive_name);

    let winrar_path = r"C:\Program Files\WinRAR\WinRAR.exe";

    let status = Command::new(winrar_path)
        .arg("a")
        .arg("-sfx")
        .arg("-m5")
        .arg("-r")
        .arg("-dh")
        .arg(&archive_path)
        .arg(&source_folder)
        .status()?;

    if status.success() {
        MessageDialog::new()
            .set_title("Готово!")
            .set_description(&format!("SFX архив создан:\n{}", archive_path))
            .show();
    } else {
        MessageDialog::new()
            .set_title("Ошибка!")
            .set_description("Не удалось создать архив через WinRAR")
            .show();
    }

    Ok(())
}
