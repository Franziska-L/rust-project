//! # MenuBar Sample
//!
//! This sample demonstrates how to use Menus/MenuBars and MenuItems in Windows.
//!
//! /!\ This is different from the system menu bar (which are preferred) available in `gio::Menu`!

extern crate gio;
extern crate glib;
extern crate gtk;
extern crate meff;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, Label, Menu, MenuBar, MenuItem, WindowPosition, FileChooserDialog, FileChooserAction, ResponseType};
use meff::network;
use crate::util::{MEFFM};
use std::net::SocketAddr;

use std::env::args;
use gdk::Window;

mod util;

// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
#headline {
    color: blue;
    font-weight: bold;
    font-size: 32px;
}
#subheadline {
    font-size: 18px;
}
#button {
    background: white;
    border-color: white;
}
#scrollview {
    padding: 10px;
}
#frame{
    padding: 10px;
}
#entry_red {
    border-color: red;
}
#entry_gray {
    border-color: #B8B8B8;
}";

fn build_startup(main_window: &gtk::ApplicationWindow, meff: MEFFM) -> gtk::Window {
    let startup_window = gtk::Window::new(gtk::WindowType::Toplevel);
    startup_window.set_position(WindowPosition::Center);
    startup_window.set_size_request(550, 300);

    let header = gtk::HeaderBar::new();
    header.set_title(Some("Sign up"));
    startup_window.set_titlebar(Some(&header));

    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(400);

    let v_box_create = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let name_entry_create = gtk::Entry::new();
    let port_entry_create = gtk::Entry::new();

    let name_box = create_entry_with_label("Name", name_entry_create.clone());
    let port_box = create_entry_with_label("Port    ", port_entry_create.clone());

    v_box_create.pack_start(&name_box, true, true, 0);
    v_box_create.pack_start(&port_box, true, true, 0);
    v_box_create.set_margin_top(20);
    v_box_create.set_margin_bottom(20);

    let v_box_join = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let name_entry_join = gtk::Entry::new();
    let port_entry_join = gtk::Entry::new();
    let ip_entry_join = gtk::Entry::new();

    let name_box_join = create_entry_with_label("Name         ", name_entry_join.clone());
    let port_box_join = create_entry_with_label("Port            ", port_entry_join.clone());
    let ip_box_join = create_entry_with_label("IP Address", ip_entry_join.clone());

    v_box_join.pack_start(&name_box_join, true, true, 0);
    v_box_join.pack_start(&port_box_join, true, true, 0);
    v_box_join.pack_start(&ip_box_join, true, true, 0);

    stack.add_titled(&v_box_create, "create", "Create network");
    stack.add_titled(&v_box_join, "join", "Join network");

    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    let start_button = gtk::Button::new_with_label("Start");
    let cancel_button = gtk::Button::new_with_label("Cancel");

    let stack_clone = stack.clone();
    start_button.connect_clicked(clone!(@weak startup_window => move |_| {
        let meffm_clone = meff.clone();
        let current_stack = stack_clone.get_visible_child_name().unwrap().as_str().to_string().clone();
        let appl = MEFFM::new();

        if current_stack == "create" {
            let name = name_entry_create.get_text().unwrap().as_str().to_string().clone();
            let port = port_entry_create.get_text().unwrap().as_str().to_string().clone();
            set_entry_border(&name, &name_entry_create);
            set_entry_border(&port, &port_entry_create);

            if !name.is_empty() && !port.is_empty() {
                let peer = match network::startup(&name, &port, None, Box::new(appl)) {
                    Ok(p) => p,
                    Err(_e) => {
                        return;
                    } // error!("Could not join network {:?}", e);
                };
                startup_window.destroy();
            }
        } else {
            let name = name_entry_join.get_text().unwrap().as_str().to_string().clone();
            let port = port_entry_join.get_text().unwrap().as_str().to_string().clone();
            let mut ip = ip_entry_join.get_text().unwrap().as_str().to_string().clone();

            set_entry_border(&name, &name_entry_join);
            set_entry_border(&port, &port_entry_join);
            set_entry_border(&ip, &ip_entry_join);

            if !name.is_empty() && !port.is_empty() && !ip.is_empty() {

                verify_ip(&ip);

                let peer = match network::startup(&name, &port, None, Box::new(appl)) {
                    Ok(p) => p,
                    Err(_e) => {
                        return;
                    } // error!("Could not join network {:?}", e);
                };
                startup_window.destroy();
            }

            //startup_window.destroy();

            println!("{:?}  {:?}  {:?}", &name, &port, &ip);

        }
    }));

    cancel_button.connect_clicked(clone!(@weak main_window => move |_| {
        main_window.destroy();
    }));

    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    h_box.pack_start(&start_button, false, true, 0);
    h_box.pack_start(&cancel_button, false, true, 0);
    h_box.set_halign(gtk::Align::Center);
    h_box.set_valign(gtk::Align::End);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    v_box.pack_start(&stack_switcher, true, true, 0);
    v_box.pack_start(&stack, true, true, 0);
    v_box.pack_start(&h_box, false, true, 10);
    v_box.set_halign(gtk::Align::Center);
    v_box.set_margin_top(10);

    startup_window.add(&v_box);
    startup_window
}

fn verify_ip(addr: &String) {
    let ip: SocketAddr = match addr.parse::<SocketAddr>() {
        Ok(socket_addr) => socket_addr,
        Err(_) => {
            //  error!("Could not parse ip address of remote Peer");
            return;
        }
    };

}

fn set_entry_border(text: &str, entry: &gtk::Entry) {
    if text.is_empty() {
        gtk::WidgetExt::set_widget_name(entry, "entry_red");
    } else {
        gtk::WidgetExt::set_widget_name(entry, "entry_gray");
    }
}


fn create_entry_with_label(text: &str, entry: gtk::Entry) -> gtk::Box {
    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    let label = Label::new(Some(&text));

    h_box.pack_start(&label, false, true, 0);
    h_box.pack_end(&entry, false, true, 0);

    h_box
}

fn build_ui(application: &gtk::Application, meff: MEFFM) {
    let main_window = ApplicationWindow::new(application);
    let startup_window = build_startup(&main_window, meff);

    main_window.set_title("MEFF");
    main_window.set_position(WindowPosition::Center);
    main_window.set_size_request(600, 600);

    let v_box_window = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let h_box_window = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let v_box2 = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    main_window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");

    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    quit.connect_activate(clone!(@weak main_window => move |_| {
        main_window.destroy();
    }));

    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    // It isn't available directly through gdk::ModifierType, since it has
    // different values on different platforms.
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    let label = Label::new(Some("MEFF"));
    let label2 = Label::new(Some("Music Entertainment For Friends"));

    gtk::WidgetExt::set_widget_name(&label, "headline");
    gtk::WidgetExt::set_widget_name(&label2, "subheadline");

    let button = gtk::Button::new_with_label("Seach music");
    //FOR CSS
    gtk::WidgetExt::set_widget_name(&button, "button1");

    let upload_button = gtk::Button::new_with_label("Upload music");

    let dialog = FileChooserDialog::new(Some("Open File"), Some(&main_window), FileChooserAction::Open);
    dialog.add_button("_Cancel", ResponseType::Cancel);
    dialog.add_button("_Open", ResponseType::Accept);

    upload_button
        .connect_clicked(move |_| {
        dialog.run();
        let file = dialog.get_filename();
        match file {
            Some(file) =>  {
                println!("{}", file.into_os_string().into_string().unwrap())
            },
            _ => {},
        }
        dialog.hide();
    });

    let stream_button = gtk::Button::new_with_label("Stream music");


    let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let textbox = gtk::Entry::new();
    let h_box_label = Label::new(Some("Title"));

    let list_box = gtk::ListBoxBuilder::new().activate_on_single_click(true).build();

    for x in 0..100 {
        let mut list_box_row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        //let label = Label::new(Some("Abba"));
        let label_button = gtk::Button::new_with_label("Abba");
        gtk::WidgetExt::set_widget_name(&label_button, "button");
        hbox.pack_start(&label_button, false, false, 5);
        list_box_row.connect_activate( move |_| {
            println!("row {} return", x);
        });
        list_box_row.add(&hbox);
        list_box_row.show_all();
        list_box.add(&list_box_row);
    }

    let mut is_playing = false;

    let title_db = Label::new(Some("Your Music"));
    let controller_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let play_music = gtk::Button::new();
    let pause_music = gtk::Button::new();
    let stop_music = gtk::Button::new();

    let image_play = gtk::Image::new_from_file("src/play.png");
    let image_pause = gtk::Image::new_from_file("src/pause.png");
    let image_stop = gtk::Image::new_from_file("src/stop.png");

    play_music.set_image(Some(&image_play));
    pause_music.set_image(Some(&image_pause));
    stop_music.set_image(Some(&image_stop));

    play_music.connect_clicked(move |_| {
        println!("Clicked play");
//        let mut playing = is_playing.clone();
//        if playing {
//            play_music.set_image(Some(&image_play));
//            playing = false;
//        } else {
//            play_music.set_image(Some(&image_pause));
//            playing = true;
//        }
    });

    pause_music.connect_clicked(move |_| {
        println!("Clicked pause");
    });

    stop_music.connect_clicked(move |_| {
        println!("Clicked stop");
    });

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    gtk::WidgetExt::set_widget_name(&scrolled_window, "scrollview");
    scrolled_window.add(&list_box);
    scrolled_window.set_size_request(100, 200);
    scrolled_window.set_valign(gtk::Align::Start);

    let frame = gtk::Frame::new(Option::from("Music Control"));
    gtk::WidgetExt::set_widget_name(&frame, "frame");

    let middle_sep = gtk::Separator::new(gtk::Orientation::Vertical);

    //TODO: testen ob das bei anderen Rechnern richtig angezeigt wird
    //let volume_button = gtk::VolumeButton::new();


    controller_box.pack_start(&play_music, false, true, 0);
    controller_box.pack_start(&pause_music, false, true, 0);
    controller_box.pack_start(&stop_music, false, true, 0);
    //controller_box.pack_start(&volume_button, false, true, 0);
    controller_box.set_halign(gtk::Align::Center);
    controller_box.set_valign(gtk::Align::End);

    v_box2.pack_start(&title_db, false, true, 0);
    v_box2.pack_start(&scrolled_window, false, true, 0);
    v_box2.pack_start(&upload_button, false, true, 0);

    h_box.pack_start(&h_box_label, false, true, 0);
    h_box.pack_start(&textbox, false, true, 0);

    v_box.pack_start(&h_box, false, true, 0);
    v_box.pack_start(&button, false, true, 0);
    v_box.pack_start(&stream_button, false, true, 0);

    h_box_window.pack_start(&v_box, true, true, 10);
    h_box_window.pack_start(&middle_sep, false, false, 0);
    h_box_window.pack_start(&v_box2, true, true, 10);

    frame.add(&h_box_window);

    v_box_window.pack_start(&menu_bar, false, false, 0);
    v_box_window.pack_start(&label, false, true, 0);
    v_box_window.pack_start(&label2, false, true, 0);
    v_box_window.pack_start(&frame, true, true, 10);
    v_box_window.pack_start(&controller_box, true, true, 10);

    main_window.add(&v_box_window);
    main_window.show_all();
    startup_window.set_modal(true);
    startup_window.set_transient_for(Some(&main_window));
    startup_window.show_all();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["gtk-rs developers"]);
        p.set_website_label(Some("gtk-rs"));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_authors(&["Gtk-rs developers"]);
        p.set_title("About!");
        p.set_transient_for(Some(&main_window));
        p.run();
        p.destroy();
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("com.meef"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        // @TODO check if it is okay to create our application model here
        let meff = MEFFM::new();
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app, meff);
    });

    application.run(&args().collect::<Vec<_>>());
    print!("run");
}
