mod corefunc;

use gdk4::Texture;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, CheckButton, DropDown, Entry, EntryBuffer, Grid,
    Label, Picture,
};
use std::sync::{Arc, Mutex};

fn main() {
    let app = Application::builder().application_id("zomid").build();
    app.connect_activate(build_ui);
    app.run();
}

//Global variable for storing paths entered by user in gui element
static USER_PATHS: Mutex<[String; 2]> = Mutex::new([String::new(), String::new()]);

fn build_ui(app: &Application) {
    //keeps track of which dropdown selection is active
    let drop_tracker = Arc::new(Mutex::new(0));
    //keeps track of the state of the checkboxes
    let mut check_tracker = Arc::new(Mutex::new([false, false]));

    let ptex = match Texture::from_filename("./images/zomboid.png") {
        Ok(ptex) => ptex,

        Err(err) => {
            println!("There was an error with the image {:?}", err);
            return Default::default();
        }
    };

    let plogo = Picture::for_paintable(&ptex);

    let instructs = Label::builder()
        .label("Enter the path to your workshop directory \nExample: c:\\program files(x86)\\steam\\steamapps\\common\\workshop\\content\\108600".to_string())
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let workshop_inst = Label::builder()
        .label("Workshop Destination: ")
        .width_chars(22)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let workshop_dest = Label::builder()
        .label("")
        .width_chars(80)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let text_inst = Label::builder()
        .label("TextFile Destination: ")
        .width_chars(22)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let text_dest = Label::builder()
        .label("")
        .width_chars(80)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let pathselect = DropDown::from_strings(&["Workshop Directory", "Text File Output  "]);

    let button_1 = Button::with_label("Submit");

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

    let work_confirm = CheckButton::with_label("Confirm Path");

    let text_confirm = CheckButton::with_label("Confirm Path");

    //=======================================
    //=   ALL USER INTERACTION GOES HERE  ===
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
                0 => "Enter the path to your workshop directory \nExample: c:\\program files(x86)\\steam\\steamapps\\common\\workshop\\content\\108600".to_string(),
                1 => "Enter the path you want your text file saved \nExample: c:\\Desktop\\".to_string(),
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

    enterbox.connect_activate({
        let workshop_dest = workshop_dest.clone();
        let text_dest = text_dest.clone();
        move |enterbox| {
            let index = *drop_tracker.lock().unwrap();
            let path_buffs = Arc::clone(&path_buffs);
            let user_paths = &mut USER_PATHS.lock().unwrap();
            user_paths[index] = enterbox.text().to_string();
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

    work_confirm.connect_toggled({
        let check_tracker = Arc::clone(&mut check_tracker);
        move |_work_confirm| {
            let mut checker = check_tracker.lock().unwrap();
            checker[0] = match &checker[0] {
                false => true,
                true => false,
            };

            if checker[0] && checker[1] == true {
                let val = true;
                println!("USER IS READY TO EXECUTE!, val => {:?}", val);
            } else {
                println!("{:?}, {:?}", &checker[0], &checker[1])
            }
        }
    });

    text_confirm.connect_toggled({
        let check_tracker = Arc::clone(&mut check_tracker);
        move |_text_confirm| {
            let mut checker = check_tracker.lock().unwrap();
            checker[1] = match &checker[1] {
                false => true,
                true => false,
            };

            if checker[0] && checker[1] == true {
                let val = true;
                println!("USER IS READY TO EXECUTE!, val => {:?}", val);
            } else {
                println!("{:?}, {:?}", &checker[0], &checker[1]);
            }
        }
    });
    //=======================================
    //=   Beginning to build the grid     ===
    //=======================================

    let grid_0 = Grid::new();

    grid_0.attach(&plogo, 1, 0, 3, 3);
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

    let vbox = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    vbox.append(&grid_0);

    let window = ApplicationWindow::builder()
        .title("ZoMID")
        .application(app)
        .child(&vbox)
        .build();

    window.show();
}
