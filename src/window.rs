/* window.rs
 *
 * Copyright 2023 Kai
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::{prelude::*, glib::{clone}, *};
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use arboard::Clipboard;

mod imp {

    use std::cell::Cell;
    use super::*;

    /// Contains all the buttons and variables the application needs.
    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/kaii_lb/hgui/gtk/window.ui")]
    pub struct HguiRustWindow {
        #[template_child]
        pub convert_button: TemplateChild<Button>,
        #[template_child]
        pub hijri_spin_button: TemplateChild<SpinButton>,
        #[template_child]
        pub gregorian_spin_button: TemplateChild<SpinButton>,
        #[template_child]
        pub start_year_spin_button: TemplateChild<SpinButton>,
        #[template_child]
        pub end_year_spin_button: TemplateChild<SpinButton>,
        #[template_child]
        pub change_between_button: TemplateChild<Button>,
        #[template_child]
        pub switch_mode_button: TemplateChild<Button>,
        #[template_child]
        pub copy_button: TemplateChild<Button>,
        #[template_child]
        pub array_list_box: TemplateChild<ListBox>,
        #[template_child]
        pub array_dialog: TemplateChild<Dialog>,
        #[template_child]
        pub gregorian_entry: TemplateChild<Entry>,
        #[template_child]
        pub hijri_entry: TemplateChild<Entry>,
        //#[template_child]
        //pub dialog_title: TemplateChild<gtk::Label>,
        #[template_child]
        pub end_year_adjustment: TemplateChild<Adjustment>,
        #[template_child]
        pub start_year_adjustment: TemplateChild<Adjustment>,
        #[template_child]
        pub error_dialog: TemplateChild<MessageDialog>,

        pub is_date: Cell<bool>,
        pub is_array: Cell<bool>,
        pub converting_to_gregorian: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HguiRustWindow {
        const NAME: &'static str = "HguiRustWindow";
        type Type = super::HguiRustWindow;
        type ParentType = adw::ApplicationWindow;

        /// Install the actions to the app for **much** easier access.
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("win.convert_to_hijri", None, move |win, _, _| {
                win.on_gregorian_value_changed();
            });

            klass.install_action("win.convert_to_gregorian", None, move |win, _, _| {
                win.on_hijri_value_changed();
            });

            klass.install_action("win.switch_mode", None, move |win, _, _| {
                win.switch_mode();
            });
            
            klass.install_action("win.convert", None, move |win, _, _| {
                win.convert();
            });
            
            klass.install_action("win.setup", None, move |win, _, _| {
                win.setup();
            });

            klass.install_action("win.copy", None, move |win, _, _| {
                win.copy();
            });

            klass.install_action("win.change_between", None, move |win, _, _| {
                win.change_between();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }


    impl ObjectImpl for HguiRustWindow {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
    
            // Setup
            let obj = self.obj();
            obj.setup();
        }
    }
    impl WidgetImpl for HguiRustWindow {}
    impl WindowImpl for HguiRustWindow {}
    impl ApplicationWindowImpl for HguiRustWindow {}
    impl AdwApplicationWindowImpl for HguiRustWindow {}
}

glib::wrapper! {
    pub struct HguiRustWindow(ObjectSubclass<imp::HguiRustWindow>)
        @extends Widget, Window, ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
} 

impl HguiRustWindow {
    pub fn new<P: IsA<Application>>(application: &P) -> Self {
        //glib::Object::new(&[("application", application)])

        let win: HguiRustWindow = glib::Object::builder()
            .property("application", application)
            .build();

        win.setup();

        win.switch_mode();
        win.switch_mode();
        win.switch_mode();
                
        win
    }

    
    /// Sets up the app by connecting buttons with their respective signals.
    fn setup(&self) {
        //let convert_button = self.imp().convert_button.get();

        let hijri_spin_button = self.imp().hijri_spin_button.get();
        let gregorian_spin_button = self.imp().gregorian_spin_button.get();

        let switch_mode_button = self.imp().switch_mode_button.get();
        let convert_button = self.imp().convert_button.get();
        let copy_button = self.imp().copy_button.get();
        let change_between_button = self.imp().change_between_button.get();

        let array_list_box = self.imp().array_list_box.get();
        
        let gregorian_entry = self.imp().gregorian_entry.get();
        let hijri_entry = self.imp().hijri_entry.get();

        let error_dialog = self.imp().error_dialog.get();

        hijri_spin_button.connect_value_changed(
            clone!(
            #[strong(rename_to = win)] self,
            move |_| {
                win.set_converting_to_gregorian(true);
                win.on_hijri_value_changed();
            }
        ));

        gregorian_spin_button.connect_value_changed(
            clone!(
            #[strong(rename_to = win)] self,
            move|_| {
                win.set_converting_to_gregorian(false);
                win.on_gregorian_value_changed();
            }
        ));

        gregorian_entry.connect_activate(
            clone!(
            #[strong(rename_to = win)] self,
            move |_| {
                win.set_converting_to_gregorian(false);
                win.on_gregorian_value_changed();
            }
        ));

        gregorian_entry.connect_has_focus_notify(
            clone!(
            #[strong(rename_to = win)] self,
            move |_| {
                win.set_converting_to_gregorian(false);
            }
        ));

        hijri_entry.connect_activate(
            clone!(
            #[strong(rename_to = win)] self,
            move |_| {
                win.set_converting_to_gregorian(true);
                win.on_hijri_value_changed();
            }
        ));

        hijri_entry.connect_has_focus_notify(
            clone!(
            #[strong(rename_to = win)] self,
            move |_| {
                win.set_converting_to_gregorian(true);
            }
        ));

        array_list_box.connect_row_selected(
            clone!(
            #[strong(rename_to = win)] self,
            move |_,_| {
                win.copy_list_box_row();
            }
        ));

        error_dialog.connect_response(
            clone!(
                #[weak(rename_to = win)] self,
                move |_,_| {
                    win.close_error_dialog();
                }
            )
        );

        change_between_button.set_action_name(Some("win.change_between"));

        switch_mode_button.set_action_name(Some("win.switch_mode"));
        
        convert_button.set_action_name(Some("win.convert"));

        copy_button.set_action_name(Some("win.copy"));
    }

    /// Copies the value of the list box row the user selected (clicked on)
    /// to the clipboard.
    fn copy_list_box_row(&self){
        if self.imp().array_list_box.selected_row() == None {
            return;
        }

        // Gets the label of the selected list box row and _extracts_ the string from that 
        let widget = self.imp().array_list_box.selected_row().unwrap().child().and_dynamic_cast_ref::<Label>().unwrap().to_owned();
        let string = widget.label().to_string().to_owned();
        let other_string = string.split("-> ").to_owned().last().unwrap().split("</span>").to_owned();

        let final_string = other_string.into_iter().next().unwrap();

        //eprintln!("{}", final_string);

        self.clipboard().set_text(&final_string);
        // let mut clipboard_pasta = ClipboardContext::new().unwrap();
        // clipboard_pasta.set_contents(final_string.to_owned()).unwrap();
        // drop(clipboard_pasta);
        let mut clipboard_arboard = Clipboard::new().unwrap();
        clipboard_arboard.set_text(final_string).unwrap();
    }
    
    /// Closes the error dialog, pretty self-explanatory.
    /// (it actually hides it but uk same thing basically)    
    fn close_error_dialog(self) {
        self.imp().error_dialog.hide();
    }

    /// Handles the change in value of the gregorian spin-button or entry.
    /// Aka takes in the value of the hijri_spin_button or entry and converts it 
    /// accordingly without the user having to manually click the convert button.
    fn on_hijri_value_changed(&self){
        if false == self.imp().is_date.get() {
            let hijri_spin_button = self.imp().hijri_spin_button.get();
            let gregorian_spin_button = self.imp().gregorian_spin_button.get();

            let hijri_date: i32 = hijri_spin_button.value_as_int();

            // This is basically a formula to convert from hijri to gregorian.
            // It basically says that to convert from hijri to gregorian you 
            // take the hijri date, multiply by the length of a hijri year (354.3 days)
            // divided by the length of the gregorian year (365.25 days)
            // and then add 622 to it.  
            let gregorian_date: i32 = (((hijri_date as f32) * (354.3/365.25)) + 622.0).floor() as i32;

            gregorian_spin_button.set_value(gregorian_date as f64);
            
            //println!("converted to gregorian with value: {gregorian_date}");
        }
        else {
            let hijri_entry = self.imp().hijri_entry.get();
            let gregorian_entry = self.imp().gregorian_entry.get();

            hijri_entry.activate();

            let hijri_date = hijri_entry.buffer().text();

            // Remove and common date dividers from the input string and get the correct variables from it
            //let hijri_iter: Vec<&str>= hijri_date.split(&['-', '/', ',', ' '][..]).collect();

            let hijri_iter: Vec<&str> = hijri_date.split(&['-', '/', ',', ' '][..]).collect();

            let error_dialog = self.imp().error_dialog.get();

            if hijri_iter.len() < 3 || hijri_iter.len() > 3 {
                error_dialog.show();
                return;
            }

            if hijri_iter[0].is_empty() || hijri_iter[0].is_empty() || hijri_iter[0].is_empty() {
                error_dialog.show();
                return;
            }
            
            if hijri_iter[0].chars().any(char::is_alphabetic) || hijri_iter[1].chars().any(char::is_alphabetic) || hijri_iter[2].chars().any(char::is_alphabetic) {
                error_dialog.show();
                return;
            }

            
            let hijri_array: [i32;3] = [hijri_iter[0].parse().unwrap(),hijri_iter[1].parse().unwrap(),hijri_iter[2].parse().unwrap()]; 
            
            let gregorian_date = self.convert_hijri_date_gregorian(hijri_array[0],hijri_array[1],hijri_array[2]);

            gregorian_entry.buffer().set_text((gregorian_date[0].to_string() + "/" + gregorian_date[1].to_string().as_str() + "/" + gregorian_date[2].to_string().as_str()).as_str());
            
            // eprintln!("converted {hijri_date} hijri to gregorian is {:?}", gregorian_date);
        }
    }

    /// Handles the change in value of the gregorian spin-button or entry.
    /// Aka takes in the value of the gregorian_spin_button or entry and converts it 
    /// accordingly without the user having to manually click the convert button.
    fn on_gregorian_value_changed(&self){
        if false == self.imp().is_date.get() {
            let hijri_spin_button = self.imp().hijri_spin_button.get();
            let gregorian_spin_button = self.imp().gregorian_spin_button.get();
    
            let gregorian_date: i32 = gregorian_spin_button.value_as_int();
    
            // This is basically a formula to convert from gregorian to hijri.
            // It basically says that to convert from gregorian to hijri you 
            // take the gregorian date, then remove 622 from it and then 
            // multiply by the length of a gregorian year (365.25 days)
            // and divide it by the length of the hijri year (354.3 days) 
            let hijri_date: i32 = (((gregorian_date as f32) - 622.0) * (365.25/354.3)).ceil() as i32;
    
            hijri_spin_button.set_value(hijri_date as f64);
            
            // eprintln!("converted to hijri with value: {hijri_date}");
        }
        else {
            let hijri_entry = self.imp().hijri_entry.get();
            let gregorian_entry = self.imp().gregorian_entry.get();

            gregorian_entry.activate();

            let gregorian_date = gregorian_entry.buffer().text();
            
            // Remove and common date dividers from the input string and get the correct variables from it
            //let gregorian_iter: Vec<&str>= gregorian_date.split(&['-', '/', ',', ' '][..]).collect();

            let gregorian_iter: Vec<&str> = gregorian_date.split(&['-', '/', ',', ' '][..]).collect();
                        
            let error_dialog = self.imp().error_dialog.get();
            
            if gregorian_iter.len() < 3 || gregorian_iter.len() > 3 {
                error_dialog.show();
                return;
            }

            if gregorian_iter[0].is_empty() || gregorian_iter[0].is_empty() || gregorian_iter[0].is_empty() {
                error_dialog.show();
                return;
            }
            
            if gregorian_iter[0].chars().any(char::is_alphabetic) || gregorian_iter[1].chars().any(char::is_alphabetic) || gregorian_iter[2].chars().any(char::is_alphabetic) {
                error_dialog.show();
                return;
            }
            
            let gregorian_array: [i32;3] = [gregorian_iter[0].parse().unwrap(),gregorian_iter[1].parse().unwrap(),gregorian_iter[2].parse().unwrap()]; 

            let hijri_date = self.convert_gregorian_date_to_hijri(gregorian_array[0], gregorian_array[1], gregorian_array[2]);

            hijri_entry.buffer().set_text((hijri_date[0].to_string() + "/" + hijri_date[1].to_string().as_str() + "/" + hijri_date[2].to_string().as_str()).as_str());
            
            // eprintln!("converted {gregorian_date} gregorian to hijri is {:?}", hijri_date);
        }
    }

    
    /// Automatically gets the value of whatever entry/spin-button the user used last
    /// and convert it to the appropriate value (hijri to gregorian and vice versa).
    /// This is used by the convert button.
    fn convert(&self) {
        if self.imp().is_array.get(){
            // convert then show the dialog containing the values converted.

            self.convert_array();
            
            let dialog = self.imp().array_dialog.get();

            dialog.set_transient_for(Some(self));

            //dialog.set_hide_on_close(true);

            dialog.show();

            return;
        }
        
        if self.imp().converting_to_gregorian.get() {
            self.on_hijri_value_changed();
        }
        else {
            self.on_gregorian_value_changed();
        }
    }

    /// Handles the switch mode button click. Checks for a bunch of variables 
    /// and switches the mode of the app accordingly.
    /// Available modes are Normal, Array, and Date.
    fn switch_mode(&self) {
        let switch_mode_button = self.imp().switch_mode_button.get();
        let change_between_button = self.imp().change_between_button.get();
        let copy_button = self.imp().copy_button.get();

        let hijri_entry = self.imp().hijri_entry.get();
        let gregorian_entry = self.imp().gregorian_entry.get();

        let hijri_spin_button = self.imp().hijri_spin_button.get();
        let gregorian_spin_button = self.imp().gregorian_spin_button.get(); 
        let start_year_spin_button = self.imp().start_year_spin_button.get();
        let end_year_spin_button = self.imp().end_year_spin_button.get(); 

        let modes = ["Normal", "Array", "Date"];
        
        let mut next_index = 0;

        for current_index in 0..modes.len() {
            if switch_mode_button.label().unwrap() == modes[current_index].to_string(){
                next_index = current_index + 1;
            }

            // limit index to the length of the modes 
            if modes.len() == next_index {
                next_index = 0;
            } 

            // eprintln!("next index in for loop is {next_index}");
        }

        if next_index == 0 {
            change_between_button.set_sensitive(false);

            gregorian_entry.set_visible(false);
            hijri_entry.set_visible(false);

            gregorian_spin_button.set_visible(true);
            hijri_spin_button.set_visible(true);

            start_year_spin_button.set_visible(false);
            end_year_spin_button.set_visible(false);

            copy_button.set_sensitive(true);

            self.set_date(false);
            self.imp().is_array.set(false);

            // eprintln!("setting mode to normal");
        }
        else if next_index == 1 {
            change_between_button.set_sensitive(true);

            // don't want them to feel left out :cry:
            gregorian_entry.set_visible(false);
            hijri_entry.set_visible(false);

            gregorian_spin_button.set_visible(false);
            hijri_spin_button.set_visible(false);

            start_year_spin_button.set_visible(true);
            end_year_spin_button.set_visible(true);

            copy_button.set_sensitive(false);

            self.set_date(false); // not really needed but just in case
            self.imp().is_array.set(true);
            self.imp().converting_to_gregorian.set(true);

            // eprintln!("setting mode to array");
        }
        else if next_index == 2 {
            change_between_button.set_sensitive(false);

            gregorian_entry.set_visible(true);
            hijri_entry.set_visible(true);

            gregorian_spin_button.set_visible(false);
            hijri_spin_button.set_visible(false);

            start_year_spin_button.set_visible(false);
            end_year_spin_button.set_visible(false);
            
            copy_button.set_sensitive(true);

            self.set_date(true);
            self.imp().is_array.set(false);

            hijri_entry.activate();
            
            // eprintln!("setting mode to date");
        }

        switch_mode_button.set_label(modes[next_index]);        
    }

    /// Sets the _converting_to_gregorian_ bool to _state_.
    /// Used for checking what the app should convert to.
    /// Aka if it should be converting from gregorian to hijri 
    /// or hijri to gregorian.
    fn set_converting_to_gregorian(&self, state: bool){
        self.imp().converting_to_gregorian.set(state);
        // eprintln!("is hijri is now {state}");
    }

    /// Sets the _is_date_ bool to _state_.
    /// Used for checking is the app should be 
    /// converting full dates or just years.
    /// Aka dd/MM/yyyy or just yyyy
    fn set_date(&self, state: bool){
        self.imp().is_date.set(state);
        // eprintln!("is date is now {state}");
    }

    /// Changes between hijri and gregorian in the _Array_ Mode. 
    /// Basically just switches the adjustments of the spin_buttons while in that mode.
    fn change_between(&self){
        self.imp().converting_to_gregorian.set(!self.imp().converting_to_gregorian.get());

        let start_year_adjustment = self.imp().start_year_adjustment.get();
        let end_year_adjustment = self.imp().end_year_adjustment.get();

        // Set the adjustments correctly so it no invalid values can be input
        if self.imp().converting_to_gregorian.get() {

            start_year_adjustment.set_lower(0.0);
            start_year_adjustment.set_upper(9999.0);
            end_year_adjustment.set_lower(0.0);
            end_year_adjustment.set_upper(9999.0);
        
            start_year_adjustment.set_value(0.0);
            end_year_adjustment.set_value(0.0);
        }
        else {
            start_year_adjustment.set_lower(622.0);
            start_year_adjustment.set_upper(9999.0);
            end_year_adjustment.set_lower(622.0);
            end_year_adjustment.set_upper(9999.0);
            
            start_year_adjustment.set_value(622.0);
            end_year_adjustment.set_value(622.0);
        }
    }

	/// Copies the result of the conversion to the clipboard.
    /// Meaning it checks if _converting_to_gregorian_ and _is_date_
    /// and copies accordingly.
	fn copy(&self){
        let mut _string: String = "".to_string();
            
        if true == self.imp().converting_to_gregorian.get() {
            if true == self.imp().is_date.get() {
                _string = self.imp().gregorian_entry.get().text().to_string();
            }
            else {
                _string = self.imp().gregorian_spin_button.get().value().to_string();				
            }
        }
        else {
            if true == self.imp().is_date.get() {
                _string = self.imp().hijri_entry.get().text().to_string();
            }
            else {
                _string = self.imp().hijri_spin_button.get().value().to_string();				
            }
        }

        // eprintln!("The string to be copied is : {}", _string);
        
        self.clipboard().set_text(&_string);
        // let mut clipboard_pasta: WindowsClipboardContext = ClipboardContext::new().unwrap();
        // clipboard_pasta.set_contents(_string.to_owned()).unwrap();
        // drop(clipboard_pasta);
		let mut clipboard_arboard = Clipboard::new().unwrap();
        clipboard_arboard.set_text(_string).unwrap();
	}

    /// Converts the years fed into the spin_buttons in _Array_ mode 
    /// between hijri and gregorian, then adds them to the [ListBox]
    /// showing them when the dialog is opened by clicking the convert button. 
    fn convert_array(&self){
        let start_date = self.imp().start_year_spin_button.get().value_as_int();
        let end_date = self.imp().end_year_spin_button.get().value_as_int();

        let array_list_box = self.imp().array_list_box.get();
        //let dialog_title = self.imp().dialog_title.get();
		let array_dialog = self.imp().array_dialog.get();

        // Delete previous entries the list box
        while let Some(row) = array_list_box.last_child() {
            array_list_box.remove(&row);
        }

        // if we are converting to gregorian then iterate over the dates
        // and convert them to gregorian
        if self.imp().converting_to_gregorian.get() {
            for i in start_date..end_date + 1 {
                //array.push(self.convert_hijri_date_gregorian(1, 1, i)[2]);

                let row = ListBoxRow::new();
                
                let string = (i as f32 * (354.3 / 365.25) + 622.0).ceil().to_string().as_str().to_owned();

                let final_string = i.to_string() + " -> " + string.as_str();

                let label = Label::new(Some(""));
                label.set_margin_bottom(8);
                label.set_margin_top(8);
                label.set_markup(("<span size='12pt'>".to_owned() + &final_string + "</span>").as_str());

                row.set_child(Some(&label));

                array_list_box.append(&row);
            }

            //dialog_title.set_label("Hijri To Gregorian");            
            array_dialog.set_title(Some("Hijri To Gregorian"));
        }
        // else if we are converting to hijri then iterate over the dates
        // and convert them to hijri
        else {
            for i in start_date..end_date + 1{
                //array.push(self.convert_gregorian_date_to_hijri(1, 1, i)[2]);
                let row = ListBoxRow::new();

                let string = ((i as f32 - 622.0) * (365.25 / 354.3)).ceil().to_string().as_str().to_owned();

                let final_string = i.to_string() + " -> " + string.as_str();

                let label = Label::new(Some(""));
                label.set_margin_bottom(8);
                label.set_margin_top(8);
                label.set_markup(("<span size='12pt'>".to_owned() + &final_string + "</span>").as_str());

                row.set_child(Some(&label));

                array_list_box.append(&row);
            }  
            
            //dialog_title.set_label("Gregorian To Hijri");            
            array_dialog.set_title(Some("Gregorian To Hijri"));
        }

        //eprintln!("Processing array done, should work...hopefully...maybe...probs not");
    }


    /// Converts a full gregorian date to hijri. So from dd/MM/yyyy in gregorian 
    /// to dd/MM/yyyy in hijri. How it works? I have no ducking idea its
    /// either black magic and/or devine intervention.
    fn convert_gregorian_date_to_hijri(&self, mut day: i32, month: i32, year: i32) -> [i32;3]{
        day += 1;
        let jd1 = self.int_part((1461.0 * ((year as f32) + 4800.0 + self.int_part(((month as f32) - 14.0) / 12.0))) / 4.0);
        let jd2 = self.int_part((367.0 * ((month as f32) - 2.0 - 12.0 * (self.int_part(((month as f32) - 14.0) / 12.0)))) / 12.0);
        let jd3 = self.int_part((3.0 * (self.int_part(((year as f32) + 4900.0 + self.int_part(((month as f32) - 14.0) / 12.0)) / 100.0))) / 4.0);
        let jd = jd1 + jd2 - jd3 + (day as f32) - 32075.0;

        let mut l = jd - 1948440.0 + 10632.0;
        let n = self.int_part((l - 1.0) / 10631.0);
        l = l - 10631.0 * n + 354.0;

        let j1 = (self.int_part((10985.0 - l) / 5316.0)) * (self.int_part((50.0 * l) / 17719.0));
        let j2 = (self.int_part(l / 5670.0)) * (self.int_part((43.0 * l) / 15238.0));
        let j = j1 + j2;

        let l1 = (self.int_part((30.0 - j) / 15.0)) * (self.int_part((17719.0 * j) / 50.0));
        let l2 = (self.int_part(j / 16.0)) * (self.int_part((15238.0 * j) / 43.0));
        l = l - l1 - l2 + 29.0; 

        let m = self.int_part((24.0 * l) / 709.0);
        let y = 30.0 * n + j - 30.0;
        let d = l - self.int_part((709.0 * m) / 24.0);

        [d as i32,m as i32,y as i32]
    }

    /// Converts a full hijri date to gregorian. So from dd/MM/yyyy in hijri 
    /// to dd/MM/yyyy in gregorian. How it works? I have no ducking idea its
    /// either black magic and/or devine intervention.
    fn convert_hijri_date_gregorian(&self, mut day: i32, month: i32, year: i32) -> [i32;3] {
        day -= 1;
        let jd1 = self.int_part((11.0 * (year as f32)+ 3.0) / 30.0);
        let jd2 = self.int_part(((month as f32) - 1.0) / 2.0);
        let jd = jd1 + 354.0 * (year as f32) + 30.0 * (month as f32) - jd2 + (day as f32) + 1948440.0 - 385.0;

        let mut l = jd + 68569.0;
        let n = self.int_part((4.0 * l) / 146097.0);
        l = l - self.int_part((146097.0 * n + 3.0) / 4.0);
        let i = self.int_part((4000.0 * (l + 1.0)) / 1461001.0);
        l = l - self.int_part((1461.0 * i) / 4.0) + 31.0;
        let j = self.int_part((80.0 * l) / 2447.0);
        
        let d = l - self.int_part((2447.0 * j) / 80.0);
        
        l = self.int_part(j / 11.0);

        let m = j + 2.0 - 12.0 * l;
        let y = 100.0 * (n - 49.0) + i + l;

        [d as i32, m as i32, y as i32]
    }

    /// No idea. Nope. You go figure it out. 
    fn int_part(&self, number: f32) -> f32{
        if number < -0.0000001 {
            (number - 0.0000001).ceil()
        }
        else {
            (number + 0.0000001).floor()
        }
    }
}
