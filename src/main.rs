#![windows_subsystem = "windows"]

mod corefunc;
use gdk4::{Display, Texture, RGBA};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, CheckButton, CssProvider, DropDown, Entry,
    EntryBuffer, Grid, Label, Picture,
};
use std::fs;
use std::sync::{Arc, Mutex};
//Global variable for storing paths entered by user in gui element
static USER_PATHS: Mutex<[String; 2]> = Mutex::new([String::new(), String::new()]);

fn main() {
    let app = Application::builder().application_id("zomid").build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    //=====================================================
    //=   Initial variables needed by multiple widgets  ===
    //=====================================================

    let drop_tracker = Arc::new(Mutex::new(0));

    let mut check_tracker = Arc::new(Mutex::new([false, false]));

    let mut is_ready = Arc::new(Mutex::new(false));

    let ptex = match Texture::from_filename("./images/zomboid.png") {
        Ok(ptex) => ptex,

        Err(err) => {
            println!("There was an error with the image {:?}", err);
            return Default::default();
        }
    };

    //=======================================================
    //=   Construction of Buttons/Enterbox/Dropdowns/etc  ===
    //=======================================================

    let plogo = Picture::for_paintable(&ptex);

    let instructs = Label::builder()
        .label("Enter the path to your workshop directory \nExample: c:\\program files(x86)\\steam\\steamapps\\common\\workshop\\content\\108600".to_string())
        .width_chars(80)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    instructs.set_widget_name("instructs");

    let workshop_inst = Label::builder()
        .label("Workshop Destination: ")
        .width_chars(20)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    workshop_inst.set_widget_name("workshop_inst");

    let workshop_dest = Label::builder()
        .label("")
        .width_chars(80)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    workshop_dest.set_widget_name("workshop_dest");

    let text_inst = Label::builder()
        .label("TextFile Destination: ")
        .width_chars(22)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    text_inst.set_widget_name("text_inst");

    let text_dest = Label::builder()
        .label("")
        .width_chars(80)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    text_dest.set_widget_name("text_dest");

    let pathselect = DropDown::from_strings(&["Workshop Directory", "Text File Output  "]);

    pathselect.set_widget_name("pathselect");

    let button_0 = Button::with_label("Exit");

    button_0.set_widget_name("button_0");

    let button_1 = Button::with_label("Submit"); //Attach trigger to submit button

    button_1.set_widget_name("button_1");

    let button_2 = Button::with_label("Close");

    button_2.set_widget_name("button_2");

    let path_buffs = Arc::new(Mutex::new(vec![
        EntryBuffer::new(None::<String>),
        EntryBuffer::new(None::<String>),
    ]));

    let enterbox = Entry::builder()
        .width_chars(50)
        .margin_top(1)
        .margin_bottom(1)
        .margin_start(5)
        .margin_end(5)
        .build();

    enterbox.set_widget_name("enterbox");

    let work_confirm = CheckButton::with_label("Confirm Path");

    work_confirm.set_widget_name("work_confirm");

    let text_confirm = CheckButton::with_label("Confirm Path");

    text_confirm.set_widget_name("text_confirm");

    let information = Label::builder()
        .label(String::new())
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    information.set_widget_name("information");

    //=======================================
    //=   User Interaction Section 1      ===
    //=======================================

    //Signal when dropdown is opened to toggle which instructions the user sees and what which path
    //to write to
    pathselect.connect_selected_item_notify({
        let instructs = instructs.clone();
        let enterbox = enterbox.clone();
        let path_buffs = Arc::clone(&path_buffs);
        let drop_tracker = Arc::clone(&drop_tracker);
        move |pathselect| {
            let index = pathselect.selected();
            let new_text = match index {
                0 => "Enter the path to your workshop directory \nExample: C:\\program files(x86)\\steam\\steamapps\\common\\workshop\\content\\108600".to_string(),
                1 => "Enter the path you want your text file saved \nExample: C:\\Desktop\\".to_string(),
                _ => "Error".to_string(),
            };

            instructs.set_label(&new_text);

            let path_buff_sel= {
                let path_buffs = path_buffs.lock().unwrap();
                match index {
                0 => path_buffs[0].clone(),
                1 => path_buffs[1].clone(),
                _ => EntryBuffer::new(None::<String>),
                }
            };
            *drop_tracker.lock().unwrap() = index as usize;
            enterbox.set_buffer(&path_buff_sel);
        }
    });

    button_0.connect_clicked({
        let app_ref = app.clone();
        move |_button_0| {
            app_ref.quit();
        }
    });

    button_1.connect_clicked({
        let workshop_dest = workshop_dest.clone();
        let text_dest = text_dest.clone();
        let enterbox = enterbox.clone();
        let drop_tracker = drop_tracker.clone();
        let path_buffs = path_buffs.clone();
        move |_button_1| {
            let index = *drop_tracker.lock().unwrap();
            let path_buffs = Arc::clone(&path_buffs);
            let user_paths = &mut USER_PATHS.lock().unwrap();
            user_paths[index] = match index {
                0 => enterbox.text().to_string(),
                1 => {
                    let text = "/Zomid.txt";
                    enterbox.text().to_string() + &text
                }
                _ => String::from("error with entrybox"),
            };
            workshop_dest.set_label(&user_paths[0]);
            text_dest.set_label(&user_paths[1]);
            let path_buff_sel = {
                let _path_buffs = path_buffs.lock().unwrap();
                match index {
                    _ => EntryBuffer::new(None::<String>),
                }
            };
            enterbox.set_buffer(&path_buff_sel);
        }
    });

    button_2.connect_clicked({
        let app_ref = app.clone();
        move |_button_2| {
            app_ref.quit();
        }
    });

    enterbox.connect_activate({
        let workshop_dest = workshop_dest.clone();
        let text_dest = text_dest.clone();
        move |enterbox| {
            let index = *drop_tracker.lock().unwrap();
            let path_buffs = Arc::clone(&path_buffs);
            let user_paths = &mut USER_PATHS.lock().unwrap();
            user_paths[index] = match index {
                0 => enterbox.text().to_string(),
                1 => {
                    let text = "/Zomid.txt";
                    enterbox.text().to_string() + &text
                }
                _ => String::from("error with entrybox"),
            };
            workshop_dest.set_label(&user_paths[0]);
            text_dest.set_label(&user_paths[1]);
            let path_buff_sel = {
                let _path_buffs = path_buffs.lock().unwrap();
                match index {
                    _ => EntryBuffer::new(None::<String>),
                }
            };
            enterbox.set_buffer(&path_buff_sel);
        }
    });

    //==================================================================
    //=  Begining to build grids and construct boxes and windows     ===
    //==================================================================

    let grid_0 = Grid::new();

    grid_0.attach(&plogo, 1, 1, 1, 1);
    grid_0.attach(&button_0, 4, 0, 1, 1);
    grid_0.attach(&instructs, 1, 3, 1, 1);
    instructs.set_xalign(0.0);
    grid_0.attach(&enterbox, 1, 4, 3, 1);
    grid_0.attach(&pathselect, 0, 4, 1, 1);
    grid_0.attach(&button_1, 4, 4, 1, 1);
    grid_0.attach(&workshop_inst, 0, 5, 1, 1);
    grid_0.attach(&workshop_dest, 1, 5, 3, 1);
    grid_0.attach(&text_inst, 0, 6, 1, 1);
    grid_0.attach(&text_dest, 1, 6, 3, 1);
    grid_0.attach(&work_confirm, 4, 5, 1, 1);
    grid_0.attach(&text_confirm, 4, 6, 1, 1);
    workshop_inst.set_xalign(0.0);
    workshop_dest.set_xalign(0.0);
    text_inst.set_xalign(0.0);
    text_dest.set_xalign(0.0);
    pathselect.set_margin_top(10);
    pathselect.set_margin_bottom(5);

    let grid_1 = Grid::new();
    grid_1.attach(&information, 0, 1, 4, 2);

    let vbox = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    vbox.set_widget_name("vbox");
    vbox.append(&grid_0);

    let pbox = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    pbox.set_widget_name("pbox");
    pbox.append(&grid_1);

    let window = ApplicationWindow::builder()
        .title("ZoMID")
        .application(app)
        .child(&vbox)
        .build();

    window.set_widget_name("window");
    window.set_deletable(false);
    window.set_visible(true);

    let progress = ApplicationWindow::builder()
        .title("ZoMID")
        .application(app)
        .child(&pbox)
        .build();

    progress.set_widget_name("progress");

    window.set_deletable(false);

    window.set_visible(true);
    //=======================================
    //=   User Interaction Section 2      ===
    //=======================================

    text_confirm.connect_toggled({
        let window = window.clone();
        let progress = progress.clone();
        let check_tracker = Arc::clone(&mut check_tracker);
        let is_ready = Arc::clone(&mut is_ready);
        move |_text_confirm| {
            let mut checker = check_tracker.lock().unwrap();
            let mut ready_check = is_ready.lock().unwrap();
            checker[1] = match &checker[1] {
                false => true,
                true => false,
            };

            if checker[0] && checker[1] == true {
                let val = true;
                println!("USER IS READY TO EXECUTE!, val => {:?}", val);
                *ready_check = val;
                window.set_visible(false);
                progress.set_visible(true);
            }
        }
    });

    work_confirm.connect_toggled({
        let window = window.clone();
        let progress = progress.clone();
        let check_tracker = Arc::clone(&mut check_tracker);
        let is_ready = Arc::clone(&mut is_ready);
        move |_work_confirm| {
            let mut checker = check_tracker.lock().unwrap();
            let mut ready_check = is_ready.lock().unwrap();
            checker[0] = match &checker[0] {
                false => true,
                true => false,
            };

            if checker[0] && checker[1] == true {
                let val = true;
                println!("USER IS READY TO EXECUTE!, val => {:?}", val);
                *ready_check = val;
                window.set_visible(false);
                progress.set_visible(true);
            }
        }
    });

    //Below signal activates upon window visiblity USE FOR EXECUTION OF COREFUNC
    progress.connect_visible_notify({
        let grid_1 = grid_1.clone();
        let button_2 = button_2.clone();
        let information = information.clone();
        move |_progress| {
            let mut complete_text: String = String::new();
            let user_paths: &[String; 2] = &mut USER_PATHS.lock().unwrap();
            let sourcevec: Vec<String> = corefunc::pathcollect(&user_paths[0]).unwrap();
            let idvec: Vec<String> = corefunc::workidbuild(&user_paths[0]).unwrap();
            let modidsvec: Vec<String> = corefunc::modidpathcollecter(sourcevec.clone()).unwrap();
            let mapnamevec: Vec<String> = corefunc::mapnamecollect(sourcevec.clone()).unwrap();
            information.set_label("Processing your request... [\\]");
            complete_text.push_str("Here are the needed ids for your installed mods! \n\n");

            information.set_label("Processing your request... [|]");
            for workshopid in idvec {
                complete_text.push_str(&workshopid);
                complete_text.push_str(";");
            }

            information.set_label("Processing your request... [/]");
            complete_text.push_str("\n\n");

            for modinfo in modidsvec {
                complete_text.push_str(&corefunc::idscollect(modinfo.clone()).unwrap());
                complete_text.push_str(";");
            }

            information.set_label("Processing your request... [-]");
            complete_text.push_str("\n\n");

            for names in mapnamevec {
                complete_text.push_str(&names);
                complete_text.push_str(";");
            }

            information.set_label("Processing your request... [\\]");
            let _ = fs::write(user_paths[1].clone(), complete_text);

            information.set_label("Your text file is created! click the button to quit! [✔ ]");
            grid_1.attach(&button_2, 4, 1, 1, 1);
        }
    });
}
