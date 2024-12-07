use iced::{Application, Command, Element, Settings, executor, Theme, Length};
use iced::widget::{Column, Text, Button, TextInput, Row, Container};

mod document;
mod layers;
mod rendering;
mod ui;

use document::Document;
use layers::{RasterLayer, VectorLayer, Scene3DLayer};
use layers::vector::VectorShape;
use rendering::renderer::Renderer;

fn main() -> iced::Result {
    println!("Arcturus Studio starting with Iced...");
    ArcturusApp::run(Settings::default())
}

struct ArcturusApp {
    document: Document,
    renderer: Renderer,
    name_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    NewDocument,
    AddRasterLayer,
    AddVectorLayer,
    Add3DLayer,
    NameChanged(String),
    ExportDocument,
}

impl Application for ArcturusApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let document = Document::new("New Project", 800, 600);
        let renderer = Renderer::new(800, 600);

        (
            ArcturusApp {
                document,
                renderer,
                name_input: String::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        format!("Arcturus Studio - {}", self.document.name)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewDocument => {
                self.document = Document::new(&self.name_input, 800, 600);
                self.renderer = Renderer::new(800, 600);
            }
            Message::AddRasterLayer => {
                let layer = RasterLayer::new(800, 600);
                self.document.add_layer(Box::new(layer));
            }
            Message::AddVectorLayer => {
                let mut layer = VectorLayer::new();
                // Add a sample rectangle
                layer.add_shape(VectorShape::Rectangle {
                    x: 100.0,
                    y: 100.0,
                    width: 200.0,
                    height: 150.0,
                    stroke_width: 2.0,
                    fill_color: [255, 0, 0, 128],
                    stroke_color: [0, 0, 0, 255],
                });
                self.document.add_layer(Box::new(layer));
            }
            Message::Add3DLayer => {
                let layer = Scene3DLayer::new();
                self.document.add_layer(Box::new(layer));
            }
            Message::NameChanged(name) => {
                self.name_input = name;
                self.document.name = self.name_input.clone();
            }
            Message::ExportDocument => {
                // TODO: Implement document export
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let header = Text::new(&self.document.name)
            .size(40);

        let name_input = TextInput::new(
            "Enter project name...",
            &self.name_input)
            .on_input(Message::NameChanged)
            .padding(10);

        let new_doc_button = Button::new(Text::new("New Document"))
            .padding(10)
            .on_press(Message::NewDocument);

        let add_raster_button = Button::new(Text::new("Add Raster Layer"))
            .padding(10)
            .on_press(Message::AddRasterLayer);

        let add_vector_button = Button::new(Text::new("Add Vector Layer"))
            .padding(10)
            .on_press(Message::AddVectorLayer);

        let add_3d_button = Button::new(Text::new("Add 3D Layer"))
            .padding(10)
            .on_press(Message::Add3DLayer);

        let export_button = Button::new(Text::new("Export"))
            .padding(10)
            .on_press(Message::ExportDocument);

        let toolbar = Row::new()
            .spacing(10)
            .push(new_doc_button)
            .push(add_raster_button)
            .push(add_vector_button)
            .push(add_3d_button)
            .push(export_button);

        let layer_list = Column::new()
            .spacing(5)
            .push(Text::new("Layers:").size(20))
            .push(Text::new(format!("Layer count: {}", self.document.layer_count())));

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(header)
            .push(name_input)
            .push(toolbar)
            .push(layer_list);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}
