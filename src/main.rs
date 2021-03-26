use gtk::prelude::*;
use relm::{Component, ContainerWidget, Widget};
use relm_derive::{widget, Msg};

pub struct Model {
    relm: relm::Relm<Win>,
}

#[derive(Msg, Debug)]
pub enum Msg {
    RowClicked,
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        let r = self.model.relm.clone();
        self.widgets
            .tv
            .connect_row_activated(move |_tv, sort_path, _col| {
                println!("emitting relm event");
                r.stream().emit(Msg::RowClicked);
            });

        let liststore = gtk::ListStore::new(&[
            String::static_type(),
            u32::static_type(),
            String::static_type(),
        ]);

        let col1 = gtk::TreeViewColumnBuilder::new().title("col1").build();
        let col1_cell = gtk::CellRendererTextBuilder::new().build();
        col1.pack_start(&col1_cell, true);
        col1.add_attribute(&col1_cell, "text", 0);
        self.widgets.tv.append_column(&col1);

        let col2 = gtk::TreeViewColumnBuilder::new().title("col2").build();
        let col2_cell = gtk::CellRendererTextBuilder::new().build();
        col2.pack_start(&col2_cell, true);
        col2.add_attribute(&col2_cell, "text", 2);
        self.widgets.tv.append_column(&col2);

        self.widgets.tv.set_model(Some(&liststore));

        for i in 0..100000 {
            dbg!(i);
            liststore.insert_with_values(
                None,
                &[0, 1, 2],
                &[
                    &format!("hi {}", i).to_value(),
                    &i.to_value(),
                    &format!("bye {}", i).to_value(),
                ],
            );
            if i % 100 == 0 {
                while gtk::events_pending() {
                    gtk::main_iteration();
                }
            }
        }
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        Model { relm: relm.clone() }
    }

    fn update(&mut self, event: Msg) {
        dbg!(event);
    }

    view! {
        gtk::Window {
            gtk::ScrolledWindow {
                #[name="tv"]
                gtk::TreeView {}
            }
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
