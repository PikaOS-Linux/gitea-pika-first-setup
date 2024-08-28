// GTK crates
use adw::prelude::*;
use adw::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;



use duct::cmd;

const REBOOT_PROG: &str = r###"
#! /bin/bash
userdel -r -f pikaos
reboot
"###;

pub fn final_carousel(first_setup_carousel: &adw::Carousel) {
    let first_setup_final_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    let first_setup_final_box_text = adw::StatusPage::builder()
        .icon_name("emblem-favorite")
        .title(t!("first_setup_final_box_text_title"))
        .description(t!("first_setup_final_box_text_description"))
        .build();

    let first_setup_start_button = gtk::Button::builder()
        .label(t!("first_setup_reboot_button_label"))
        .halign(Align::Center)
        .build();

    first_setup_start_button.add_css_class("suggested-action");
    first_setup_start_button.add_css_class("pill");

    first_setup_final_box.append(&first_setup_final_box_text);
    first_setup_final_box.append(&first_setup_start_button);

    first_setup_carousel.append(&first_setup_final_box);

    first_setup_start_button.connect_clicked( move |_| {
        let _ = cmd!("/usr/lib/pika/pika-first-setup-gtk4/scripts/pika-sudo.sh", "bash", "-c", REBOOT_PROG).read();
    });
}
