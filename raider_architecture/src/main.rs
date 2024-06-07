slint::slint! {
    import { LineEdit, GroupBox, VerticalBox, ComboBox, Button, StandardButton } from "std-widgets.slint";

    ////////////////////////////////////////////////////////////////////////////
    //
    // Other component
    //
    ////////////////////////////////////////////////////////////////////////////

    export global Callbacks {
        callback new_project_button_clicked();
        callback load_project_button_clicked(image,string);
        callback home_button_clicked();
        callback edit_project_button_clicked(string);
        callback open_project_button_clicked();
        callback fe_ok_clicked(string);
        callback fe_cancel_clicked();
        callback fe_input_edited();
        callback process_button_clicked(string,string,string,string,string,string,string,string,string,string,string,string,string,string);
        callback nat_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback nwa_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback nbat_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback na_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback nbed_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback nfs_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback hab_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback lir_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback kit_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback sbat_edited(string,string,string,string,string,string,string,string,string,string,string);
        callback sbed_edited(string,string,string,string,string,string,string,string,string,string,string);
    }

    component Header-Button {
        in property <string> text;
        in property <color> color;
        in property <color> background;
        in property <color> hover;
        in property <string> button-comand: "void";
        in property <string> project_name;
        Rectangle {
            background: ta.has-hover ? root.hover : root.background;
            border-radius: ta.has-hover ? 10px : 5px;
            animate background,border-radius {
                 duration: 100ms;
            }
            ta := TouchArea {
                clicked => {
                    if button-comand == "new" {
                        Callbacks.new_project_button_clicked();
                    }
                    else if button-comand == "home" {
                        Callbacks.home_button_clicked();
                    }
                    else if button-comand == "edit" {
                        Callbacks.edit_project_button_clicked(root.project_name);
                    }
                    else if button-comand == "open" {
                        Callbacks.open_project_button_clicked();
                    }
                }
            }
        }
        Text {
            text: root.text;
            color: root.color;
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // Project Manager
    //
    ////////////////////////////////////////////////////////////////////////////

    component Project-Minia {
        in property <color> background;
        in property <color> hover;
        in property <image> minia;
        in property <string> callback_type;
        in property <string> name;
        Image {
            source: minia;
            width: 100%;
            height: 100%;
            Rectangle {
                background: ta.has-hover ? root.hover : root.background;
                opacity: ta.has-hover ? 20% : 0%;
                animate background,opacity {
                    duration: 250ms;
                }
                ta := TouchArea {
                    clicked => {
                        if callback_type == "new" {
                            Callbacks.new_project_button_clicked();
                        }
                        else if callback_type == "load" {
                            Callbacks.load_project_button_clicked(minia,name);
                        }
                    }
                }
            }
        }
    }

    component ProjectManager {
        in property <image> minia1;
        in property <image> minia2;
        in property <image> minia3;
        in property <image> minia4;
        in property <image> minia5;
        in property <image> minia6;
        in property <image> minia7;
        in property <image> minia8;
        in property <string> callback_type1;
        in property <string> callback_type2;
        in property <string> callback_type3;
        in property <string> callback_type4;
        in property <string> callback_type5;
        in property <string> callback_type6;
        in property <string> callback_type7;
        in property <string> callback_type8;
        in property <string> name1;
        in property <string> name2;
        in property <string> name3;
        in property <string> name4;
        in property <string> name5;
        in property <string> name6;
        in property <string> name7;
        in property <string> name8;
        Rectangle {
            background: #777;
            GridLayout {
                Row {
                    Rectangle {
                        background: #fff;
                        height: 25px;
                        width: 100%;
                        Header-Button {
                            text: "New";
                            color: #000;
                            background: #eee;
                            hover: #bbb;
                            x: 10px;
                            button-comand: "new";
                        }
                        Header-Button {
                            text: "Open";
                            color: #000;
                            background: #eee;
                            hover: #bbb;
                            x: 50px;
                            button-comand: "open";
                        }
                        Header-Button {
                            text: "Refresh";
                            color: #000;
                            background: #eee;
                            hover: #bbb;
                            x: 100px;
                            button-comand: "home";
                        }
                    }
                }
                Row {
                    GridLayout {
                        padding: 25px;
                        spacing: 25px;
                        Row {
                            HorizontalLayout {
                                spacing: 25px;
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: @image-url("src/assets/add_minia.png");
                                    callback_type: "new";
                                }
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia1;
                                    callback_type: callback_type1;
                                    name: root.name1;
                                }
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia2;
                                    callback_type: callback_type2;
                                    name: root.name2;
                                }
                            }
                        }
                        Row {
                            HorizontalLayout {
                                spacing: 25px;
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia3;
                                    callback_type: callback_type3;
                                    name: root.name3;
                                }
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia4;
                                    callback_type: callback_type4;
                                    name: root.name4;
                                }
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia5;
                                    callback_type: callback_type5;
                                    name: root.name5;
                                }
                            }
                        }
                        Row {
                            HorizontalLayout {
                                spacing: 25px;
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia6;
                                    callback_type: callback_type6;
                                    name: root.name6;
                                }
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia7;
                                    callback_type: callback_type7;
                                    name: root.name7;
                                }
                                Project-Minia {
                                    background: #222;
                                    hover: #000;
                                    minia: minia8;
                                    callback_type: callback_type8;
                                    name: root.name8;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // Constraint Selection
    //
    ////////////////////////////////////////////////////////////////////////////

    component ConstraintSelection {

        in property <bool> process_button_enabled;
        in property <string> process_message;
        in property <bool> err_process_visible;
        in property <string> err_process_message;
        in property <string> save_name;
        in property <bool> home_visible;

        in property <string> text1;
        in property <string> text2;
        in property <string> text3;
        in property <string> text4;
        in property <string> text5;
        in property <string> text6;
        in property <string> text7;
        in property <string> text8;
        in property <string> text9;
        in property <string> text10;
        in property <string> text11;
        in property <string> combo1;
        in property <string> combo2;
        Rectangle {
            background: #777;
            GridLayout {
                Row {
                    Rectangle {
                        background: #fff;
                        height: 25px;
                        width: 100%;
                        Header-Button {
                            visible: root.home_visible;
                            text: "Home";
                            color: #000;
                            background: #eee;
                            hover: #bbb;
                            x: 10px;
                            button-comand: "home";
                        }
                    }
                }
                Row {
                    Rectangle {
                        GroupBox
                        {
                            VerticalBox
                            {
                                // Choose the number of living room
                                Text {
                                    text: "Choose the number of living room";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                nat := LineEdit
                                {
                                    placeholder-text: "0, 1, ...";
                                    text: root.text1;
                                    edited => {
                                        Callbacks.nat_edited(self.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,hab.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the number of kitchen
                                Text {
                                    text: "Choose the number of kitchen";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                nwa := LineEdit
                                {
                                    placeholder-text: "0, 1, ...";
                                    text: root.text2;
                                    edited => {
                                        Callbacks.nwa_edited(nat.text,self.text,nbat.text,na.text,nbed.text,nfs.text,hab.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the number of bathroom
                                Text {
                                    text: "Choose the number of bathroom";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                nbat := LineEdit
                                {
                                    placeholder-text: "0, 1, ...";
                                    text: root.text3;
                                    edited => {
                                        Callbacks.nbat_edited(nat.text,nwa.text,self.text,na.text,nbed.text,nfs.text,hab.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the number of toilets
                                Text {
                                    text: "Choose the number of toilets";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                na := LineEdit
                                {
                                    placeholder-text: "0, 1, ...";
                                    text: root.text4;
                                    edited => {
                                        Callbacks.na_edited(nat.text,nwa.text,nbat.text,self.text,nbed.text,nfs.text,hab.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the number of bedroom
                                Text {
                                    text: "Choose the number of bedroom";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                nbed := LineEdit
                                {
                                    placeholder-text: "0, 1, ...";
                                    text: root.text5;
                                    edited => {
                                        Callbacks.nbed_edited(nat.text,nwa.text,nbat.text,na.text,self.text,nfs.text,hab.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the number of free room
                                Text {
                                    text: "Choose the number of free room";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                nfs := LineEdit
                                {
                                    placeholder-text: "0, 1, ...";
                                    text: root.text6;
                                    edited => {
                                        Callbacks.nfs_edited(nat.text,nwa.text,nbat.text,na.text,nbed.text,self.text,hab.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                            }
                            VerticalBox
                            {
                                // Choose your house orientation
                                Text {
                                    text: "Choose your house orientation";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                c := ComboBox {
                                    model: ["North", "West", "East", "South"];
                                    current-value: root.combo1;
                                }
                                // Choose your types of doors
                                Text {
                                    text: "Choose your types of doors";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                ca := ComboBox {
                                    model: ["I don't care", "Sliding Doors", "Hinged Doors"];
                                    current-value: root.combo2;
                                }
                                // Choose the superficy of habitation (m²)
                                Text {
                                    text: "The superficy of habitation (m²)";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                hab := LineEdit
                                {
                                    placeholder-text: "10, 100, 42, ...";
                                    text: root.text7;
                                    edited => {
                                        Callbacks.hab_edited(nat.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,self.text,lir.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the superficy of living room (m²)
                                Text {
                                    text: "Choose the superficy of living room (m²)";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                lir := LineEdit
                                {
                                    placeholder-text: "10, 100, 42, ...";
                                    text: root.text8;
                                    edited => {
                                        Callbacks.lir_edited(nat.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,hab.text,self.text,kit.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the superficy of kitchen (m²)
                                Text {
                                    text: "Choose the superficy of kitchen (m²)";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                kit := LineEdit
                                {
                                    placeholder-text: "10, 100, 42, ...";
                                    text: root.text9;
                                    edited => {
                                        Callbacks.kit_edited(nat.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,hab.text,lir.text,self.text,sbat.text,sbed.text);
                                    }
                                }
                                // Choose the superficy of bathroom (m²)
                                Text {
                                    text: "Choose the superficy of bathroom (m²)";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                sbat := LineEdit
                                {
                                    placeholder-text: "10, 100, 42, ...";
                                    text: root.text10;
                                    edited => {
                                        Callbacks.sbat_edited(nat.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,hab.text,lir.text,kit.text,self.text,sbed.text);
                                    }
                                }
                                // Choose the superficy of bedroom (m²)
                                Text {
                                    text: "Choose the superficy of bedroom (m²)";
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                sbed := LineEdit
                                {
                                    placeholder-text: "10, 100, 42, ...";
                                    text: root.text11;
                                    edited => {
                                        Callbacks.sbed_edited(nat.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,hab.text,lir.text,kit.text,sbat.text,self.text);
                                    }
                                }
                            }
                            VerticalBox
                            {
                                width: 150px;
                                height: 50px;
                                Button
                                {
                                    enabled: root.process_button_enabled;
                                    text: root.process_message;
                                    clicked => {
                                        Callbacks.process_button_clicked(root.save_name,nat.text,nwa.text,nbat.text,na.text,nbed.text,nfs.text,c.current-value,ca.current-value,hab.text,lir.text,kit.text,sbat.text,sbed.text)
                                    }
                                }
                                Text {
                                    text: root.err_process_message;
                                    visible: root.err_process_visible;
                                    wrap: word-wrap;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // Result Display
    //
    ////////////////////////////////////////////////////////////////////////////

    component ResultDisplay {
        in property <image> plan;
        in property <string> name;

        Rectangle {
            background: #777;
            GridLayout {
                Row {
                    Rectangle {
                        background: #fff;
                        height: 25px;
                        width: 100%;
                        Header-Button {
                            text: "Home";
                            color: #000;
                            background: #eee;
                            hover: #bbb;
                            x: 10px;
                            button-comand: "home";
                        }
                        Header-Button {
                            text: "Edit";
                            color: #000;
                            background: #eee;
                            hover: #bbb;
                            x: 50px;
                            button-comand: "edit";
                            project_name: root.name;
                        }
                    }
                }
                Row {
                    Rectangle {
                        VerticalBox {
                            padding: 50px;
                            Text {
                                text: root.name;
                            }
                            Rectangle {
                                background: #fff;
                                Image {
                                    width: 90%;
                                    source: root.plan;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // File Explorer
    //
    ////////////////////////////////////////////////////////////////////////////

    component FileExplorer {
        in property <bool> err_visible;
        in property <string> err_message;

        Rectangle {
            x: 25px;
            y: 50px;
            background: #222;
            width: 308px;
            height: 172px;
            Rectangle {
                VerticalLayout {
                    padding: 10px;
                    spacing: 10px;
                    alignment: center;
                    Text {
                        text: "Save number";
                    }
                    filepath := LineEdit {
                        width:280px;
                        height: 35px;
                        placeholder-text: "1, 2, 3, ...";
                        edited => {
                            Callbacks.fe_input_edited();
                        }
                    }
                    Text {
                        text: root.err_message;
                        visible: root.err_visible;
                    }
                }
            }
            StandardButton {
                x: 190px;
                y: 130px;
                kind: ok;
                clicked => {
                    Callbacks.fe_ok_clicked(filepath.text)
                }
            }
            StandardButton {
                x: 240px;
                y: 130px;
                kind: cancel;
                clicked => {
                    Callbacks.fe_cancel_clicked()
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // Global App
    //
    ////////////////////////////////////////////////////////////////////////////

    export component App inherits Window {
        title: "Raider's Architecture";
        icon: @image-url("src/assets/icon.png");
        width: 1024px;
        height: 640px;

        in property <bool> pm_visible: true;
        in property <bool> cs_visible: false;
        in property <bool> rd_visible: false;
        in property <bool> fe_visible: false;

        // Project Manager
        in property <image> minia1: @image-url("src/assets/void.png");
        in property <image> minia2: @image-url("src/assets/void.png");
        in property <image> minia3: @image-url("src/assets/void.png");
        in property <image> minia4: @image-url("src/assets/void.png");
        in property <image> minia5: @image-url("src/assets/void.png");
        in property <image> minia6: @image-url("src/assets/void.png");
        in property <image> minia7: @image-url("src/assets/void.png");
        in property <image> minia8: @image-url("src/assets/void.png");

        in property <string> callback_type1: "void";
        in property <string> callback_type2: "void";
        in property <string> callback_type3: "void";
        in property <string> callback_type4: "void";
        in property <string> callback_type5: "void";
        in property <string> callback_type6: "void";
        in property <string> callback_type7: "void";
        in property <string> callback_type8: "void";

        in property <string> name1;
        in property <string> name2;
        in property <string> name3;
        in property <string> name4;
        in property <string> name5;
        in property <string> name6;
        in property <string> name7;
        in property <string> name8;

        //Constraint Selection
        in property <string> process_message: "Process";
        in property <bool> err_process_visible: true;
        in property <string> err_process_message: "The house surface must be given by you";
        in property <bool> process_button_enabled: false;
        in property <string> save_name: "void";
        in property <bool> home_visible: true;

        in property <string> text1;
        in property <string> text2;
        in property <string> text3;
        in property <string> text4;
        in property <string> text5;
        in property <string> text6;
        in property <string> text7;
        in property <string> text8;
        in property <string> text9;
        in property <string> text10;
        in property <string> text11;
        in property <string> combo1: "North";
        in property <string> combo2: "I don't care";

        // Result Display
        in property <string> project_name;
        in property <image> plan;

        // File Explorer
        in property <bool> err_visible: false;
        in property <string> err_message;

        ProjectManager {
            visible: root.pm_visible;
            width: 100%;
            height: 100%;

            minia1: root.minia1;
            minia2: root.minia2;
            minia3: root.minia3;
            minia4: root.minia4;
            minia5: root.minia5;
            minia6: root.minia6;
            minia7: root.minia7;
            minia8: root.minia8;
            callback_type1: root.callback_type1;
            callback_type2: root.callback_type2;
            callback_type3: root.callback_type3;
            callback_type4: root.callback_type4;
            callback_type5: root.callback_type5;
            callback_type6: root.callback_type6;
            callback_type7: root.callback_type7;
            callback_type8: root.callback_type8;
            name1: root.name1;
            name2: root.name2;
            name3: root.name3;
            name4: root.name4;
            name5: root.name5;
            name6: root.name6;
            name7: root.name7;
            name8: root.name8;
        }
        ConstraintSelection {
            visible: root.cs_visible;
            width: 100%;
            height: 100%;

            process_button_enabled: root.process_button_enabled;
            process_message: root.process_message;
            err_process_message: root.err_process_message;
            err_process_visible: root.err_process_visible;
            save_name: root.save_name;
            home_visible: root.home_visible;

            text1: root.text1;
            text2: root.text2;
            text3: root.text3;
            text4: root.text4;
            text5: root.text5;
            text6: root.text6;
            text7: root.text7;
            text8: root.text8;
            text9: root.text9;
            text10: root.text10;
            text11: root.text11;
            combo1: root.combo1;
            combo2: root.combo2;
        }
        ResultDisplay {
            visible: root.rd_visible;
            width: 100%;
            height: 100%;

            plan: root.plan;
            name: root.project_name;
        }
        FileExplorer {
            visible: root.fe_visible;
            width: 100%;
            height: 100%;
            err_message: root.err_message;
            err_visible: root.err_visible;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
//
// Using
//
////////////////////////////////////////////////////////////////////////////////

use raiders_architecture::house::display::*;
use raiders_architecture::house::room::Facing;
use raiders_architecture::house::room::RoomType;
use raiders_architecture::house::save_load::*;
use raiders_architecture::ui::check::*;
use raiders_architecture::ui::image::*;
use raiders_architecture::ui::random::*;
use raiders_architecture::ui::saves::*;
use slint::{ComponentHandle, Image, SharedString};
use std::fs;
use std::fs::File;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////
//
// Main function
//
////////////////////////////////////////////////////////////////////////////////

fn main() {
    // Load App
    let app = App::new().unwrap();

    ////////////////////////////////////////////////////////////////////////////
    //
    // Load before
    //
    ////////////////////////////////////////////////////////////////////////////
    let weak = app.as_weak().unwrap();
    let images_to_load = load_images("src/assets/Saves/".to_string());
    let mut cpt = 1;
    for file in images_to_load {
        let filename = SharedString::from(get_filename(&file).unwrap());
        match cpt {
            1 => {
                weak.set_minia1(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type1(SharedString::from("load"));
                weak.set_name1(filename);
            }
            2 => {
                weak.set_minia2(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type2(SharedString::from("load"));
                weak.set_name2(filename);
            }
            3 => {
                weak.set_minia3(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type3(SharedString::from("load"));
                weak.set_name3(filename);
            }
            4 => {
                weak.set_minia4(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type4(SharedString::from("load"));
                weak.set_name4(filename);
            }
            5 => {
                weak.set_minia5(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type5(SharedString::from("load"));
                weak.set_name5(filename);
            }
            6 => {
                weak.set_minia6(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type6(SharedString::from("load"));
                weak.set_name6(filename);
            }
            7 => {
                weak.set_minia7(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type7(SharedString::from("load"));
                weak.set_name7(filename);
            }
            8 => {
                weak.set_minia8(Image::load_from_path(Path::new(&file)).unwrap());
                weak.set_callback_type8(SharedString::from("load"));
                weak.set_name8(filename);
            }
            _ => break,
        }
        cpt += 1;
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // Callbacks
    //
    ////////////////////////////////////////////////////////////////////////////
    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>()
        .on_new_project_button_clicked(move || {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_text1(SharedString::new());
            weak.set_text2(SharedString::new());
            weak.set_text3(SharedString::new());
            weak.set_text4(SharedString::new());
            weak.set_text5(SharedString::new());
            weak.set_text6(SharedString::new());
            weak.set_text7(SharedString::new());
            weak.set_text8(SharedString::new());
            weak.set_text9(SharedString::new());
            weak.set_text10(SharedString::new());
            weak.set_text11(SharedString::new());
            weak.set_combo1(SharedString::from("North"));
            weak.set_combo2(SharedString::from("I don't care"));
            weak.set_save_name(SharedString::from("void"));
            weak.set_home_visible(true);
            weak.set_pm_visible(false);
            weak.set_cs_visible(true);
        });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>()
        .on_load_project_button_clicked(move |minia, pn| {
            weak.set_project_name(pn);
            weak.set_plan(minia);
            weak.set_pm_visible(false);
            weak.set_rd_visible(true);
        });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_home_button_clicked(move || {
        if !weak.get_name1().is_empty() {
            weak.set_minia1(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type1(SharedString::from("void"));
            weak.set_name1(SharedString::new());
        }
        if !weak.get_name2().is_empty() {
            weak.set_minia2(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type2(SharedString::from("void"));
            weak.set_name2(SharedString::new());
        }
        if !weak.get_name3().is_empty() {
            weak.set_minia3(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type3(SharedString::from("void"));
            weak.set_name3(SharedString::new());
        }
        if !weak.get_name4().is_empty() {
            weak.set_minia4(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type4(SharedString::from("void"));
            weak.set_name4(SharedString::new());
        }
        if !weak.get_name5().is_empty() {
            weak.set_minia5(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type5(SharedString::from("void"));
            weak.set_name5(SharedString::new());
        }
        if !weak.get_name6().is_empty() {
            weak.set_minia6(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type6(SharedString::from("void"));
            weak.set_name6(SharedString::new());
        }
        if !weak.get_name7().is_empty() {
            weak.set_minia7(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type7(SharedString::from("void"));
            weak.set_name7(SharedString::new());
        }
        if !weak.get_name8().is_empty() {
            weak.set_minia8(Image::load_from_path(Path::new("src/assets/void.png")).unwrap());
            weak.set_callback_type8(SharedString::from("void"));
            weak.set_name8(SharedString::new());
        }
        let images_to_load = load_images("src/assets/Saves/".to_string());
        let mut cpt = 1;
        for file in images_to_load {
            let filename = SharedString::from(get_filename(&file).unwrap());
            match cpt {
                1 => {
                    weak.set_minia1(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type1(SharedString::from("load"));
                    weak.set_name1(filename);
                }
                2 => {
                    weak.set_minia2(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type2(SharedString::from("load"));
                    weak.set_name2(filename);
                }
                3 => {
                    weak.set_minia3(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type3(SharedString::from("load"));
                    weak.set_name3(filename);
                }
                4 => {
                    weak.set_minia4(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type4(SharedString::from("load"));
                    weak.set_name4(filename);
                }
                5 => {
                    weak.set_minia5(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type5(SharedString::from("load"));
                    weak.set_name5(filename);
                }
                6 => {
                    weak.set_minia6(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type6(SharedString::from("load"));
                    weak.set_name6(filename);
                }
                7 => {
                    weak.set_minia7(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type7(SharedString::from("load"));
                    weak.set_name7(filename);
                }
                8 => {
                    weak.set_minia8(Image::load_from_path(Path::new(&file)).unwrap());
                    weak.set_callback_type8(SharedString::from("load"));
                    weak.set_name8(filename);
                }
                _ => break,
            }
            cpt += 1;
        }
        weak.set_rd_visible(false);
        weak.set_cs_visible(false);
        weak.set_pm_visible(true);
    });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>()
        .on_edit_project_button_clicked(move |filename| {
            let h = file_to_house(Path::new(&format!(
                "src/assets/Saves/Contrain{}.txt",
                filename
            )));
            if h.is_err() {
                panic!("Algoritm problem")
            }
            let house = h.unwrap();
            weak.set_combo1(match house.facing {
                Facing::North => SharedString::from("North"),
                Facing::South => SharedString::from("South"),
                Facing::East => SharedString::from("East"),
                Facing::West => SharedString::from("West"),
            });
            weak.set_combo2(SharedString::from("I don't care"));
            if house.width.is_some() && house.length.is_some() {
                weak.set_text7(SharedString::from(
                    (house.width.unwrap() * house.length.unwrap()).to_string(),
                ));
            } else {
                weak.set_callback_type7(SharedString::new());
            }
            let mut cpt_lr: usize = 0;
            let mut cpt_kit: usize = 0;
            let mut cpt_bath: usize = 0;
            let mut cpt_wc: usize = 0;
            let mut cpt_bed: usize = 0;
            let mut cpt_empty: usize = 0;
            let mut moy_lr: f32 = 0.0;
            let mut moy_kit: f32 = 0.0;
            let mut moy_bath: f32 = 0.0;
            let mut moy_bed: f32 = 0.0;
            for r in house.rooms {
                match r.room_type {
                    RoomType::LivingRoom => {
                        cpt_lr += 1;
                        if r.width.is_some() && r.length.is_some() {
                            moy_lr += r.width.unwrap() * r.length.unwrap();
                        }
                    }
                    RoomType::Kitchen => {
                        cpt_kit += 1;
                        if r.width.is_some() && r.length.is_some() {
                            moy_kit += r.width.unwrap() * r.length.unwrap();
                        }
                    }
                    RoomType::Bathroom => {
                        cpt_bath += 1;
                        if r.width.is_some() && r.length.is_some() {
                            moy_bath += r.width.unwrap() * r.length.unwrap();
                        }
                    }
                    RoomType::Bedroom => {
                        cpt_bed += 1;
                        if r.width.is_some() && r.length.is_some() {
                            moy_bed += r.width.unwrap() * r.length.unwrap();
                        }
                    }
                    RoomType::Empty => cpt_empty += 1,
                    RoomType::Toilet => cpt_wc += 1,
                    _ => panic!("You had edit the file..."),
                }
            }
            weak.set_text1(SharedString::from(cpt_lr.to_string()));
            weak.set_text2(SharedString::from(cpt_kit.to_string()));
            weak.set_text3(SharedString::from(cpt_bath.to_string()));
            weak.set_text4(SharedString::from(cpt_wc.to_string()));
            weak.set_text5(SharedString::from(cpt_bed.to_string()));
            weak.set_text6(SharedString::from(cpt_empty.to_string()));
            weak.set_text8(SharedString::from(
                (moy_lr / cpt_lr as f32).round().to_string(),
            ));
            weak.set_text9(SharedString::from(
                (moy_kit / cpt_kit as f32).round().to_string(),
            ));
            weak.set_text10(SharedString::from(
                (moy_bath / cpt_bath as f32).round().to_string(),
            ));
            weak.set_text11(SharedString::from(
                (moy_bed / cpt_bed as f32).round().to_string(),
            ));
            delete_file(format!("src/assets/Saves/{}.png", filename).as_str());
            delete_file("src/assets/.plan.svg");
            weak.set_process_message(SharedString::from("Process"));
            weak.set_save_name(filename);
            weak.set_process_button_enabled(true);
            weak.set_err_process_visible(false);
            weak.set_home_visible(false);
            weak.set_rd_visible(false);
            weak.set_cs_visible(true);
        });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>()
        .on_open_project_button_clicked(move || {
            weak.set_err_visible(false);
            weak.set_fe_visible(true);
        });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>()
        .on_fe_ok_clicked(move |save_number| {
            if save_exists(save_number.as_str()) {
                let filename = format!("src/assets/Saves/Save{}.png", save_number);
                weak.set_plan(Image::load_from_path(Path::new(&filename)).unwrap());
                weak.set_project_name(SharedString::from(get_filename(&filename).unwrap()));
                weak.set_fe_visible(false);
                weak.set_rd_visible(true);
            } else {
                weak.set_err_message(SharedString::from("Save number doesn't exists"));
                weak.set_err_visible(true);
            }
        });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_fe_cancel_clicked(move || {
        weak.set_fe_visible(false);
    });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_fe_input_edited(move || {
        weak.set_err_visible(false);
    });

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_nat_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_nwa_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_nbat_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_na_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_nbed_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_nfs_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_hab_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !hab_super.is_empty() && hab_super.parse::<f64>().unwrap() > 0.0 {
                if !check_surface(
                    (*hab_super).to_string(),
                    (*lir_super).to_string(),
                    (*kit_super).to_string(),
                    (*sbat_super).to_string(),
                    (*sbed_super).to_string(),
                    (*lir_number).to_string(),
                    (*kit_number).to_string(),
                    (*bath_number).to_string(),
                    (*wc_number).to_string(),
                    (*bed_number).to_string(),
                    (*free_number).to_string(),
                ) {
                    weak.set_process_button_enabled(false);
                    weak.set_err_process_message(SharedString::from("The house is too litle"));
                    weak.set_err_process_visible(true);
                } else {
                    weak.set_err_process_visible(false);
                    weak.set_process_button_enabled(true);
                }
            } else {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from(
                    "The house surface must be given by you",
                ));
                weak.set_err_process_visible(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_lir_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_kit_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_sbat_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_sbed_edited(
        move |lir_number,
              kit_number,
              bath_number,
              wc_number,
              bed_number,
              free_number,
              hab_super,
              lir_super,
              kit_super,
              sbat_super,
              sbed_super| {
            weak.set_process_message(SharedString::from("Process"));
            weak.set_err_process_visible(false);
            if !check_surface(
                (*hab_super).to_string(),
                (*lir_super).to_string(),
                (*kit_super).to_string(),
                (*sbat_super).to_string(),
                (*sbed_super).to_string(),
                (*lir_number).to_string(),
                (*kit_number).to_string(),
                (*bath_number).to_string(),
                (*wc_number).to_string(),
                (*bed_number).to_string(),
                (*free_number).to_string(),
            ) {
                weak.set_process_button_enabled(false);
                weak.set_err_process_message(SharedString::from("The house is too litle"));
                weak.set_err_process_visible(true);
            } else {
                weak.set_err_process_visible(false);
                weak.set_process_button_enabled(true);
            }
        },
    );

    let weak = app.as_weak().unwrap();
    app.global::<Callbacks>().on_process_button_clicked(
        move |project_save_name,
              living_room_number,
              kitchen_number,
              bathroom_number,
              wc_number,
              bedroom_number,
              empty_room_number,
              house_orientation,
              door_type,
              hab_super,
              living_room_super,
              kitchen_super,
              bathroom_super,
              bedroom_super| {
            if !living_room_number.chars().all(|c| c.is_ascii_digit())
                || !kitchen_number.chars().all(|c| c.is_ascii_digit())
                || !bathroom_number.chars().all(|c| c.is_ascii_digit())
                || !wc_number.chars().all(|c| c.is_ascii_digit())
                || !bedroom_number.chars().all(|c| c.is_ascii_digit())
                || !empty_room_number.chars().all(|c| c.is_ascii_digit())
                || !hab_super.chars().all(|c| c.is_ascii_digit())
                || !living_room_super.chars().all(|c| c.is_ascii_digit())
                || !kitchen_super.chars().all(|c| c.is_ascii_digit())
                || !bathroom_super.chars().all(|c| c.is_ascii_digit())
                || !bedroom_super.chars().all(|c| c.is_ascii_digit())
            {
                weak.set_process_message(SharedString::from("Failure"));
                weak.set_err_process_message(SharedString::from("A entry is not a number"));
                weak.set_err_process_visible(true);
            } else if hab_super.is_empty() {
                weak.set_process_message(SharedString::from("Failure"));
                weak.set_err_process_message(SharedString::from(
                    "The house surface must be given by you",
                ));
                weak.set_err_process_visible(true);
            } else {
                weak.set_process_message(SharedString::from("Generation..."));
                let files = load_images("src/assets/Saves/".to_string());
                let mut _save_number = String::new();
                if project_save_name == "void" {
                    if files.is_empty() {
                        _save_number = "0".to_string()
                    } else {
                        _save_number = get_filename(files[0].clone().as_str())
                            .unwrap()
                            .chars()
                            .nth(get_filename(files[0].clone().as_str()).unwrap().len() - 1)
                            .unwrap()
                            .to_string();
                    }
                } else {
                    _save_number = ((project_save_name
                        .chars()
                        .nth(project_save_name.chars().count() - 1)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .unwrap())
                        - 1)
                    .to_string();
                }
                let filename = format!(
                    "ContrainSave{}.txt",
                    _save_number.parse::<usize>().unwrap() + 1
                );
                let filepath = format!("src/assets/Saves/{}", filename);
                if Path::new(filepath.as_str()).exists() {
                    fs::remove_file(filepath.clone()).unwrap();
                }
                File::create(filepath.clone()).unwrap();

                // Init
                let total = hab_super.parse::<f64>().unwrap();
                let (length, width) = random(total);
                save_name(
                    &filename,
                    format!(
                        "House: Some({})/Some({})/{}/{}\n",
                        length,
                        width,
                        house_orientation,
                        match door_type {
                            _ if door_type == *"Sliding Doors" => "Sliding",
                            _ if door_type == *"Hinged Doors" => "Hinged",
                            _ => "None",
                        }
                    )
                    .as_str(),
                );
                // Living room
                if !living_room_number.is_empty()
                    && living_room_number.parse::<f64>().unwrap() > 0.0
                {
                    let number = living_room_number.parse::<usize>().unwrap();
                    if !living_room_super.is_empty()
                        && living_room_super.parse::<f64>().unwrap() > 0.0
                    {
                        for i in 0..number {
                            let total = living_room_super.parse::<f64>().unwrap();
                            let (length, width) = random(total);
                            save_name(
                                &filename,
                                format!(
                                    "Room: LivingRoom{}:LivingRoom/Some({})/Some({})/None\n",
                                    i, length, width
                                )
                                .as_str(),
                            );
                        }
                    } else {
                        for i in 0..number {
                            save_name(
                                &filename,
                                format!("Room: LivingRoom{i}:LivingRoom/None/None/None\n").as_str(),
                            );
                        }
                    }
                }
                // Kitchen
                if !kitchen_number.is_empty() && kitchen_number.parse::<f64>().unwrap() > 0.0 {
                    let number = kitchen_number.parse::<usize>().unwrap();
                    if !kitchen_super.is_empty() && kitchen_super.parse::<f64>().unwrap() > 0.0 {
                        for i in 0..number {
                            let total = kitchen_super.parse::<f64>().unwrap();
                            let (length, width) = random(total);
                            save_name(
                                &filename,
                                format!(
                                    "Room: Kitchen{}:Kitchen/Some({})/Some({})/None\n",
                                    i, length, width
                                )
                                .as_str(),
                            );
                        }
                    } else {
                        for i in 0..number {
                            save_name(
                                &filename,
                                format!("Room: Kitchen{i}:Kitchen/None/None/None\n").as_str(),
                            );
                        }
                    }
                }
                // Bathroom
                if !bathroom_number.is_empty() && bathroom_number.parse::<f64>().unwrap() > 0.0 {
                    let number = bathroom_number.parse::<usize>().unwrap();
                    if !bathroom_super.is_empty() && bathroom_super.parse::<f64>().unwrap() > 0.0 {
                        for i in 0..number {
                            let total = bathroom_super.parse::<f64>().unwrap();
                            let (length, width) = random(total);
                            save_name(
                                &filename,
                                format!(
                                    "Room: Bathroom{}:Bathroom/Some({})/Some({})/None\n",
                                    i, length, width
                                )
                                .as_str(),
                            );
                        }
                    } else {
                        for i in 0..number {
                            save_name(
                                &filename,
                                format!("Room: Bathroom{i}:Bathroom/None/None/None\n").as_str(),
                            );
                        }
                    }
                }
                // Bedroom
                if !bedroom_number.is_empty() && bedroom_number.parse::<f64>().unwrap() > 0.0 {
                    let number = bedroom_number.parse::<usize>().unwrap();
                    if !bedroom_super.is_empty() && bedroom_super.parse::<f64>().unwrap() > 0.0 {
                        for i in 0..number {
                            let total = bedroom_super.parse::<f64>().unwrap();
                            let (length, width) = random(total);
                            save_name(
                                &filename,
                                format!(
                                    "Room: Bedroom{}:Bedroom/Some({})/Some({})/None\n",
                                    i, length, width
                                )
                                .as_str(),
                            );
                        }
                    } else {
                        for i in 0..number {
                            save_name(
                                &filename,
                                format!("Room: Bedroom{i}:Bedroom/None/None/None\n").as_str(),
                            );
                        }
                    }
                }
                // WC
                if !wc_number.is_empty() && wc_number.parse::<f64>().unwrap() > 0.0 {
                    let number = wc_number.parse::<usize>().unwrap();
                    for i in 0..number {
                        if un_sur_deux() {
                            save_name(
                                &filename,
                                format!("Room: Toilet{i}:Toilet/Some(2)/Some(1)/None\n").as_str(),
                            );
                        } else {
                            save_name(
                                &filename,
                                format!("Room: Toilet{i}:Toilet/Some(1)/Some(2)/None\n").as_str(),
                            );
                        }
                    }
                }
                // Free Room
                if !empty_room_number.is_empty() && empty_room_number.parse::<f64>().unwrap() > 0.0
                {
                    let number = empty_room_number.parse::<usize>().unwrap();
                    for i in 0..number {
                        save_name(
                            &filename,
                            format!("Room: Empty{i}:Empty/None/None/None\n").as_str(),
                        );
                    }
                }

                ////////////////////////////////////////////////////////////////
                //
                // Algo
                //
                ////////////////////////////////////////////////////////////////
                let house = file_to_house(Path::new(&filepath));
                if house.is_err() {
                    panic!("Algorithm error")
                }
                let mut h = house.unwrap();
                let _ = h.create_rooms();
                h.create_connections();
                h.generate_walls();
                let file = File::create(format!(
                    "src/assets/Saves/PlanSave{}.txt",
                    _save_number.parse::<usize>().unwrap() + 1
                ))
                .expect("Unable to create file");
                house_to_file(&h, file).expect("Unable to write house to file");

                ////////////////////////////////////////////////////////////////
                //
                // Display
                //
                ////////////////////////////////////////////////////////////////
                create_svg(
                    file_to_house(Path::new(
                        format!(
                            "src/assets/Saves/PlanSave{}.txt",
                            _save_number.parse::<usize>().unwrap() + 1
                        )
                        .as_str(),
                    ))
                    .unwrap(),
                );
                render_svg_to_png(
                    "src/assets/.plan.svg",
                    format!(
                        "src/assets/Saves/Save{}.png",
                        _save_number.parse::<usize>().unwrap() + 1
                    )
                    .as_str(),
                );
                weak.set_plan(
                    Image::load_from_path(Path::new(
                        format!(
                            "src/assets/Saves/Save{}.png",
                            _save_number.parse::<usize>().unwrap() + 1
                        )
                        .as_str(),
                    ))
                    .unwrap(),
                );
                weak.set_project_name(SharedString::from(format!(
                    "Save{}",
                    _save_number.parse::<usize>().unwrap() + 1
                )));

                weak.set_cs_visible(false);
                weak.set_rd_visible(true);
            }
        },
    );
    // Run App
    app.run().unwrap();
}
