use eframe::egui;
use std::process::Command;

fn run_script(script: &str) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("bash /home/ja3mbc/{} > /tmp/ham_debug.log 2>&1", script))
        .env("DISPLAY", ":0")
        .env("HOME", "/home/ja3mbc")
        .spawn();
}

struct HamControl {
    status: String,
}

impl Default for HamControl {
    fn default() -> Self {
        Self {
            status: "待機中".to_string(),
        }
    }
}

impl eframe::App for HamControl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("JA3MBC HAM局 コントロール");
            ui.separator();

            ui.label("--- 起動 ---");
            if ui.button("1) WSJT-X 起動").clicked() {
                run_script("wsjtx_start.sh");
                self.status = "WSJT-X 起動".to_string();
            }
            if ui.button("2) WSJT-X 起動 (flrig（+rigctld）+WSJT-X+Hamlog+MailQSL+JT_Linker)").clicked() {
                run_script("ft8_start.sh");
                self.status = "WSJT-X 起動 (flrig（+rigctld）+WSJT-X+Hamlog+MailQSL+JT_Linker)".to_string();
            }
            if ui.button("3) RM7400 起動 (Hamlog+Remote7400+MailQSL)").clicked() {
                run_script("rm7400_start.sh");
                self.status = "RM7400 起動 (Hamlog+Remote7400+MailQSL)".to_string();
            }
            if ui.button("4) MMSSTV 起動 (MMSSTV+Hamlog+MailQSL)").clicked() {
                run_script("mmsstv_start.sh");
                self.status = "MMSSTV 起動 (MMSSTV+Hamlog+MailQSL)".to_string();
            }
            if ui.button("5) FreeDV 起動 (flrig+FD_Linker(→Hamlog)+FreeDV+MailQSL)").clicked() {
                run_script("freedv_start.sh");
                self.status = "FreeDV 起動 (flrig+FD_Linker(→Hamlog)+FreeDV+MailQSL)".to_string();
            }
            if ui.button("6) fldigi 起動 (flrig+fldigi+Hamlog+MailQSL)").clicked() {
                run_script("fldigi_start.sh");
                self.status = "fldigi 起動 (flrig+fldigi+Hamlog+MailQSL)".to_string();
            }

            ui.separator();
            ui.label("--- 停止 ---");
            if ui.button("7) WSJT-X 停止").clicked() {
                run_script("wsjtx_stop.sh");
                self.status = "WSJT-X 停止".to_string();
            }
            if ui.button("8) WSJT-X 停止 (flrig（+rigctld）+WSJT-X+Hamlog+MailQSL+JT_Linker)").clicked() {
                run_script("ft8_stop.sh");
                self.status = "WSJT-X 停止 (flrig（+rigctld）+WSJT-X+Hamlog+MailQSL+JT_Linker)".to_string();
            }
            if ui.button("9) RM7400 停止 (Hamlog+Remote7400+MailQSL)").clicked() {
                run_script("rm7400_stop.sh");
                self.status = "RM7400 停止 (Hamlog+Remote7400+MailQSL)".to_string();
            }
            if ui.button("10) MMSSTV 停止 (MMSSTV+Hamlog+MailQSL)").clicked() {
                run_script("mmsstv_stop.sh");
                self.status = "MMSSTV 停止 (MMSSTV+Hamlog+MailQSL)".to_string();
            }
            if ui.button("11) FreeDV 停止 (flrig+FD_Linker(→Hamlog)+FreeDV+MailQSL)").clicked() {
                run_script("freedv_stop.sh");
                self.status = "FreeDV 停止 (flrig+FD_Linker(→Hamlog)+FreeDV+MailQSL)".to_string();
            }
            if ui.button("12) fldigi 停止 (flrig+fldigi+Hamlog+MailQSL)").clicked() {
                run_script("fldigi_stop.sh");
                self.status = "fldigi 停止 (flrig+fldigi+Hamlog+MailQSL)".to_string();
            }

            ui.separator();
            ui.label(format!("ステータス: {}", self.status));
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 520.0])
            .with_title("HAM局 コントロール"),
        ..Default::default()
    };

    eframe::run_native(
        "HAM局 コントロール",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "noto_cjk".to_owned(),
                egui::FontData::from_static(
                    include_bytes!("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc")
                ),
            );
            fonts.families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "noto_cjk".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            Box::new(HamControl::default())
        }),
    )
}
