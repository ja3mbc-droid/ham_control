use eframe::egui;
use std::process::Command;

fn run_script(script: &str) {
    let _ = Command::new("bash")
        .arg(format!("/home/ja3mbc/{}", script))
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
            if ui.button("1) FT8モード 起動").clicked() {
                run_script("ft8_start.sh");
                self.status = "FT8モード 起動".to_string();
            }
            if ui.button("2) RM7400モード 起動").clicked() {
                run_script("rm7400_start.sh");
                self.status = "RM7400モード 起動".to_string();
            }
            if ui.button("3) WSJT-X単独 起動").clicked() {
                run_script("wsjtx_start.sh");
                self.status = "WSJT-X 起動".to_string();
            }
            if ui.button("4) MMSSTV 起動").clicked() {
                run_script("mmsstv_start.sh");
                self.status = "MMSSTV 起動".to_string();
            }
            if ui.button("5) RM7400+Hamlog+MailQSL 起動").clicked() {
                run_script("hamlog_start.sh");
                self.status = "Hamlogモード 起動".to_string();
            }
            if ui.button("6) FreeDV 起動").clicked() {
                run_script("freedv_start.sh");
                self.status = "FreeDV 起動".to_string();
            }

            ui.separator();
            ui.label("--- 停止 ---");

            if ui.button("7) FT8モード 停止").clicked() {
                run_script("ft8_stop.sh");
                self.status = "FT8モード 停止".to_string();
            }
            if ui.button("8) RM7400モード 停止").clicked() {
                run_script("rm7400_stop.sh");
                self.status = "RM7400モード 停止".to_string();
            }
            if ui.button("9) WSJT-X 停止").clicked() {
                run_script("wsjtx_stop.sh");
                self.status = "WSJT-X 停止".to_string();
            }
            if ui.button("10) MMSSTV 停止").clicked() {
                run_script("mmsstv_stop.sh");
                self.status = "MMSSTV 停止".to_string();
            }
            if ui.button("11) RM7400+Hamlog+MailQSL 停止").clicked() {
                run_script("hamlog_stop.sh");
                self.status = "RM7400+Hamlog+MailQSL 停止".to_string();
            }
            if ui.button("12) FreeDV 停止").clicked() {
                run_script("freedv_stop.sh");
                self.status = "FreeDV 停止".to_string();
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
