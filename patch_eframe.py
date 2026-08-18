import re

with open("src/main.rs", "r") as f:
    code = f.read()

# Fix update method signature
code = code.replace("fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)", "fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)")

