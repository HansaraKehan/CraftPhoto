//! Edit › Keyboard Shortcuts › Set in the dialog: a new set saves the dialog's bindings, the
//! dropdown switches sets, Save and Delete wait for OK, and Export / Summarize write the bindings
//! as the dialog shows them.

use std::cell::RefCell;
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use serde_json::{Value, json};

use crate::PhotocraftApp;
use crate::file_dialog::FileDialogAnswer;

type Written = Rc<RefCell<Vec<(String, Vec<u8>)>>>;

/// The dialog, with the save picker answering `answers` and writes recorded.
fn dialog(answers: Vec<&str>) -> (Harness<'static, PhotocraftApp>, u64, Written) {
    let written: Written = Rc::default();
    let log = written.clone();
    let mut h = Harness::builder().with_size(egui::vec2(1100.0, 760.0)).with_max_steps(64).build_eframe(move |cc| {
        PhotocraftApp::setup_context(&cc.egui_ctx, Default::default());
        let mut services = crate::Services::default();
        let (picker, _) = crate::file_dialog::fake(answers.iter().map(|a| Some(FileDialogAnswer::SaveTo((*a).into()))).collect());
        services.file_dialog = Some(picker);
        services.write = Some(Box::new(move |path: &str, bytes: &[u8]| {
            log.borrow_mut().push((path.to_string(), bytes.to_vec()));
            Ok(())
        }));
        PhotocraftApp::new(photocraft_engine::Session::new(), services)
    });
    let ctx = h.ctx.clone();
    let id = crate::menus::invoke(h.state_mut(), &ctx, "edit.keyboardShortcuts", json!({})).unwrap()["dialog"].as_u64().unwrap();
    h.run_steps(4);
    (h, id, written)
}

fn field(h: &Harness<'_, PhotocraftApp>, id: u64, name: &str) -> Value {
    h.state().ui.dialogs.iter().find(|d| d.id == id).unwrap().fields.get(name).cloned().unwrap_or(Value::Null)
}

fn set_field(h: &mut Harness<'_, PhotocraftApp>, id: u64, name: &str, v: Value) {
    h.state_mut().ui.dialog_mut(id).unwrap().fields.insert(name.into(), v);
    h.run_steps(2);
}

fn click(h: &mut Harness<'_, PhotocraftApp>, label: &str) {
    // The first match: the set row's buttons come before the dialog's own (its Cancel).
    h.get_all_by_role_and_label(egui::accesskit::Role::Button, label).next().expect("the button").click();
    h.run_steps(3);
}

#[test]
fn a_new_set_saves_the_dialogs_bindings_and_ok_keeps_it() {
    let (mut h, id, _) = dialog(vec![]);
    // An edit in the dialog, then New Set… with a name.
    set_field(&mut h, id, "overrides", json!({"edit.copy": "F9"}));
    click(&mut h, "New Set…");
    set_field(&mut h, id, "newSetName", json!("  Painting "));
    click(&mut h, "Create");
    assert_eq!(field(&h, id, "activeSet"), json!("Painting"));
    assert_eq!(field(&h, id, "sets")["Painting"]["shortcuts"], json!({"edit.copy": "F9"}));
    assert!(field(&h, id, "newSetName").is_null());
    // A bad name is refused with a message.
    click(&mut h, "New Set…");
    set_field(&mut h, id, "newSetName", json!("PhotoCraft Defaults"));
    click(&mut h, "Create");
    assert!(field(&h, id, "message").as_str().unwrap_or_default().contains("built-in"), "{:?}", field(&h, id, "message"));
    click(&mut h, "Cancel");
    // Nothing is saved before OK; OK saves the set and keeps it in use.
    assert!(h.state().session.prefs().shortcut_sets.is_empty());
    crate::dialogs::confirm(h.state_mut(), id).unwrap();
    let p = h.state().session.prefs();
    assert_eq!((p.shortcut_set.as_str(), p.shortcuts.get("edit.copy").map(String::as_str)), ("Painting", Some("F9")));
    assert_eq!(p.shortcut_sets["Painting"].shortcuts.get("edit.copy").map(String::as_str), Some("F9"));
}

#[test]
fn choosing_a_set_loads_it_and_save_and_delete_wait_for_ok() {
    let (mut h, first, _) = dialog(vec![]);
    h.state_mut().ui.dialogs.retain(|d| d.id != first);
    h.state_mut().run("edit.keyboardShortcuts", json!({"set": {"edit.copy": "F9"}, "saveSet": "A"})).unwrap();
    h.state_mut().run("edit.keyboardShortcuts", json!({"useSet": ""})).unwrap();
    let ctx = h.ctx.clone();
    let id = crate::menus::invoke(h.state_mut(), &ctx, "edit.keyboardShortcuts", json!({})).unwrap()["dialog"].as_u64().unwrap();
    h.run_steps(4);
    assert_eq!(field(&h, id, "overrides"), json!({}));
    // Choosing set A in the dropdown (as its row sets it) loads its bindings.
    set_field(&mut h, id, "activeSet", json!("A"));
    let set = field(&h, id, "sets")["A"].clone();
    set_field(&mut h, id, "overrides", set["shortcuts"].clone());
    // An edit marks it modified; Save stores it in the dialog's sets.
    set_field(&mut h, id, "overrides", json!({"edit.copy": "F8"}));
    click(&mut h, "Save");
    assert_eq!(field(&h, id, "sets")["A"]["shortcuts"]["edit.copy"], "F8");
    assert_eq!(h.state().session.prefs().shortcut_sets["A"].shortcuts["edit.copy"], "F9", "not before OK");
    click(&mut h, "Delete");
    assert_eq!((field(&h, id, "sets"), field(&h, id, "activeSet")), (json!({}), json!("")));
    crate::dialogs::confirm(h.state_mut(), id).unwrap();
    let p = h.state().session.prefs();
    assert!(p.shortcut_sets.is_empty() && p.shortcut_set.is_empty());
    assert_eq!(p.shortcuts.get("edit.copy").map(String::as_str), Some("F8"), "the bindings stay");
}

#[test]
fn export_and_summarize_write_the_dialogs_bindings() {
    let (mut h, id, written) = dialog(vec!["Mine.kys", "Mine.html"]);
    set_field(&mut h, id, "overrides", json!({"edit.copy": "F9", "tools.select.pencil": "F6"}));
    set_field(&mut h, id, "alternates", json!({"edit.copy": []}));
    let ctx = h.ctx.clone();
    click(&mut h, "Export…");
    h.state_mut().poll_file_dialog(&ctx, None);
    click(&mut h, "Summarize…");
    h.state_mut().poll_file_dialog(&ctx, None);
    let written = written.borrow();
    assert_eq!(written.len(), 2, "{:?}", written.iter().map(|w| &w.0).collect::<Vec<_>>());
    let kys = String::from_utf8(written[0].1.clone()).unwrap();
    assert!(kys.contains("pc-id=\"edit.copy\"><shortcut>F9</shortcut></command>"), "{kys}");
    assert!(kys.contains("pc-id=\"tools.select.pencil\">F6</tool>"), "tools as <tool> rows: {kys}");
    assert!(kys.contains("KeyboardCustomization/PhotoCraft Defaults="), "{kys}");
    // The file reads back with the same bindings.
    let set = photocraft_engine::kys::parse(&kys).unwrap();
    assert!(set.commands.iter().any(|c| c.id.as_deref() == Some("edit.copy") && c.shortcuts == ["F9"]));
    let html = String::from_utf8(written[1].1.clone()).unwrap();
    assert!(html.contains("<td>Copy</td><td>F9</td>") && html.contains("<h2>Edit</h2>"), "{html}");
    // Nothing changed the preferences.
    assert!(h.state().session.prefs().shortcuts.is_empty());
}
