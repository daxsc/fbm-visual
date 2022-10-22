#![windows_subsystem = "windows"]
use iced::{
    button, text_input, Alignment, Button, Column, Element, Application, Settings, Text, TextInput, Row, Color, Rectangle, Length,
    canvas::{self, Program, Frame, Geometry, Path},
    canvas::path::{self, Builder}, Command, Executor, executor, Point, Size, Canvas,
};
use noise::{Fbm, Seedable, MultiFractal, NoiseFn};
use rand::{Rng, thread_rng};

pub fn main() -> iced::Result {
    NoiseGui::run(Settings::default())
}

static USE_COLOR:bool = false;

struct NoiseGui {
    state: State,
}

#[derive(Debug)]
struct Map {
    map_cache: canvas::Cache,
    size_x: i32,
    size_y: i32,
    seed: u32,
    freq: f64,
    octaves: usize,
    lacunarity: f64,
    persistence: f64,

    // pixels: Vec<Vec<u8>>,
}

impl Map {
    // pub fn generate_data(&mut self){
    //     self.pixels.clear();
    //     let mut fbm = Fbm::new();
    //     fbm = fbm.set_seed(self.seed);
    //     fbm = fbm.set_frequency(self.freq);
    //     fbm = fbm.set_octaves(self.octaves);
    //     fbm = fbm.set_lacunarity(self.lacunarity);
    //     fbm = fbm.set_persistence(self.persistence);
    //     for y in 0..self.size_y{
    //         for x in 0..self.size_x{
    //             let noise = fbm.get([x as f64, y as f64]);
    //             if noise < -0.7{
    //                 self.pixels.push(vec![0, 255, 255]);//water
    //             }
    //             else if -0.7 < noise && noise < -0.6{
    //                 self.pixels.push(vec![127, 51, 0]);//dirt
    //             }
    //             else if -0.6 < noise && noise < -0.3{
    //                 self.pixels.push(vec![128, 128, 128]);//stone floor
    //             }
    //             else if -0.3 < noise {
    //                 self.pixels.push(vec![64, 64, 64]);//stone wall
    //             }
    //         }
    //     }
    // }
    fn new() -> Self{
        let mut m = Map {
            map_cache: canvas::Cache::new(),
            size_x: 300,
            size_y: 300,
            seed: 300,
            freq: 0.03,
            octaves: 5,
            lacunarity: 2.0,
            persistence: 0.5,
            // pixels: vec![],
        };
        // let mut fbm = Fbm::new();
        // fbm = fbm.set_seed(m.seed);
        // fbm = fbm.set_frequency(m.freq);
        // fbm = fbm.set_octaves(m.octaves);
        // fbm = fbm.set_lacunarity(m.lacunarity);
        // fbm = fbm.set_persistence(m.persistence);
        // for y in 0..m.size_y{
        //     for x in 0..m.size_x{
        //         let noise = fbm.get([x as f64, y as f64]);
        //         if noise < -0.7{
        //             m.pixels.push(vec![0, 255, 255]);//water
        //         }
        //         else if -0.7 < noise && noise < -0.6{
        //             m.pixels.push(vec![127, 51, 0]);//dirt
        //         }
        //         else if -0.6 < noise && noise < -0.3{
        //             m.pixels.push(vec![128, 128, 128]);//stone floor
        //         }
        //         else if -0.3 < noise {
        //             m.pixels.push(vec![64, 64, 64]);//stone wall
        //         }
        //     }
        // }
        return m;
    }
}

#[derive(Debug)]
struct State {
    map: Map,
    seed_input_value: String,
    seed_input_state: text_input::State,
    size_x_input_value: String,
    size_x_input_state: text_input::State,
    size_y_input_value: String,
    size_y_input_state: text_input::State,
    freq_input_value: String,
    freq_input_state: text_input::State,
    octaves_input_value: String,
    octaves_input_state: text_input::State,
    lacunarity_input_value: String,
    lacunarity_input_state: text_input::State,
    persistence_input_value: String,
    persistence_input_state: text_input::State,
    seedarrupbutton: button::State,
    seedarrdownbutton: button::State,
    freqarrupbutton: button::State,
    freqarrdownbutton: button::State,
    octavesarrupbutton: button::State,
    octavesarrdownbutton: button::State,
    lacunarityarrupbutton: button::State,
    lacunarityarrdownbutton: button::State,
    persistencearrupbutton: button::State,
    persistencearrdownbutton: button::State,
}

#[derive(Debug, Clone)]
pub enum Message{
    SizeXInputChanged(String),
    SizeYInputChanged(String),
    SeedInputChanged(String),
    FreqInputChanged(String),
    OctavesInputChanged(String),
    LacunarityInputChanged(String),
    PersistenceInputChanged(String),
    SeedArrUpPressed,
    SeedArrDownPressed,
    FreqArrUpPressed,
    FreqArrDownPressed,
    OctavesArrUpPressed,
    OctavesArrDownPressed,
    LacunarityArrUpPressed,
    LacunarityArrDownPressed,
    PersistenceArrUpPressed,
    PersistenceArrDownPressed,
}

impl State {
    pub fn new() -> State{
        let mut s = State{
            map: Map::new(),
            seed_input_value: String::new(),
            seed_input_state: text_input::State::new(),
            size_x_input_value: String::new(),
            size_x_input_state: text_input::State::new(),
            size_y_input_value: String::new(),
            size_y_input_state: text_input::State::new(),
            freq_input_value: String::new(),
            freq_input_state: text_input::State::new(),
            octaves_input_value: String::new(),
            octaves_input_state: text_input::State::new(),
            lacunarity_input_value: String::new(),
            lacunarity_input_state: text_input::State::new(),
            persistence_input_value: String::new(),
            persistence_input_state: text_input::State::new(),
            seedarrupbutton: button::State::new(),
            seedarrdownbutton: button::State::new(),
            freqarrupbutton: button::State::new(),
            freqarrdownbutton: button::State::new(),
            octavesarrupbutton: button::State::new(),
            octavesarrdownbutton: button::State::new(),
            lacunarityarrupbutton: button::State::new(),
            lacunarityarrdownbutton: button::State::new(),
            persistencearrupbutton: button::State::new(),
            persistencearrdownbutton: button::State::new(),
        };
        //update everything in input values
        s.size_x_input_value = s.map.size_x.to_string();
        s.size_y_input_value = s.map.size_y.to_string();
        s.seed_input_value = s.map.seed.to_string();
        s.freq_input_value = s.map.freq.to_string();
        if !s.freq_input_value.contains("."){s.freq_input_value.push_str(".0")}
        s.octaves_input_value = s.map.octaves.to_string();
        s.lacunarity_input_value = s.map.lacunarity.to_string();
        if !s.lacunarity_input_value.contains("."){s.lacunarity_input_value.push_str(".0")}
        s.persistence_input_value = s.map.persistence.to_string();
        if !s.persistence_input_value.contains("."){s.persistence_input_value.push_str(".0")}
        return s;
    }

    fn update(&mut self){
        self.map.map_cache.clear();
    }
}

impl Application for NoiseGui {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        return (NoiseGui { state: State::new() }, Command::none());
    }

    fn title(&self) -> String {
        String::from("NoiseGui")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message{
            Message::SeedArrDownPressed => {
                self.state.map.seed -= 1;
                self.state.seed_input_value = self.state.map.seed.to_string();
            }
            Message::SeedArrUpPressed => {
                self.state.map.seed += 1;
                self.state.seed_input_value = self.state.map.seed.to_string();
            }
            Message::FreqArrDownPressed => {
                self.state.map.freq -= 0.01;
                self.state.freq_input_value = self.state.map.freq.to_string();
            }
            Message::FreqArrUpPressed => {
                self.state.map.freq += 0.01;
                self.state.freq_input_value = self.state.map.freq.to_string();
            }
            Message::OctavesArrDownPressed => {
                self.state.map.octaves -= 1;
                self.state.octaves_input_value = self.state.map.octaves.to_string();
            }
            Message::OctavesArrUpPressed => {
                self.state.map.octaves += 1;
                self.state.octaves_input_value = self.state.map.octaves.to_string();
            }
            Message::LacunarityArrDownPressed => {
                self.state.map.lacunarity -= 0.01;
                self.state.lacunarity_input_value = self.state.map.lacunarity.to_string();
            }
            Message::LacunarityArrUpPressed => {
                self.state.map.lacunarity += 0.01;
                self.state.lacunarity_input_value = self.state.map.lacunarity.to_string();
            }
            Message::PersistenceArrDownPressed => {
                self.state.map.persistence -= 0.01;
                self.state.persistence_input_value = self.state.map.persistence.to_string();
            }
            Message::PersistenceArrUpPressed => {
                self.state.map.persistence += 0.01;
                self.state.persistence_input_value = self.state.map.persistence.to_string();
            }
            Message::SizeXInputChanged(str) => {
                if str.is_empty(){self.state.size_x_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<i64>().unwrap()) >= (u32::MAX as i64){
                        self.state.map.size_x = i32::MAX;
                        self.state.size_x_input_value = self.state.map.size_x.to_string();
                    }
                    else{
                        self.state.map.size_x = str.parse().unwrap();
                        self.state.size_x_input_value = self.state.map.size_x.to_string();}
                }
            }
            Message::SizeYInputChanged(str) => {
                if str.is_empty(){self.state.size_y_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<i64>().unwrap()) >= (u32::MAX as i64){
                        self.state.map.size_y = i32::MAX;
                        self.state.size_y_input_value = self.state.map.size_y.to_string();
                    }
                    else{
                        self.state.map.size_y = str.parse().unwrap();
                        self.state.size_y_input_value = self.state.map.size_y.to_string();}
                }
            }
            Message::SeedInputChanged(str) => {
                if str.is_empty(){self.state.seed_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<u64>().unwrap()) >= (u32::MAX as u64){
                        self.state.map.seed = u32::MAX;
                        self.state.seed_input_value = self.state.map.seed.to_string();
                    }
                    else{
                        self.state.map.seed = str.parse().unwrap();
                        self.state.seed_input_value = self.state.map.seed.to_string();}
                }
            }
            Message::FreqInputChanged(str) => {
                if str.is_empty(){self.state.freq_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<f64>().unwrap()) >= (f64::MAX){//kinda useless :(
                        self.state.map.freq = f64::MAX;
                        self.state.freq_input_value = self.state.map.freq.to_string();
                    }
                    else{
                        self.state.map.freq = str.parse().unwrap();
                        self.state.freq_input_value = self.state.map.freq.to_string();
                        if !self.state.freq_input_value.contains("."){
                            self.state.freq_input_value.push_str(".0");
                        }
                    }
                }
            }
            Message::OctavesInputChanged(str) => {
                if str.is_empty(){self.state.octaves_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<u64>().unwrap()) >= (usize::MAX as u64){
                        self.state.map.octaves = usize::MAX;
                        self.state.octaves_input_value = self.state.map.octaves.to_string();
                    }
                    else{
                        self.state.map.octaves = str.parse().unwrap();
                        self.state.octaves_input_value = self.state.map.octaves.to_string();}
                }
            }
            Message::LacunarityInputChanged(str) => {
                if str.is_empty(){self.state.lacunarity_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<f64>().unwrap()) >= (f64::MAX){//kinda useless :(
                        self.state.map.lacunarity = f64::MAX;
                        self.state.lacunarity_input_value = self.state.map.lacunarity.to_string();
                    }
                    else{
                        self.state.map.lacunarity = str.parse().unwrap();
                        self.state.lacunarity_input_value = self.state.map.lacunarity.to_string();
                        if !self.state.lacunarity_input_value.contains("."){
                            self.state.lacunarity_input_value.push_str(".0");
                        }
                    }
                }
            }
            Message::PersistenceInputChanged(str) => {
                if str.is_empty(){self.state.persistence_input_value = str;}
                else if isnum(&str){
                    if (str.parse::<f64>().unwrap()) >= (f64::MAX){//kinda useless :(
                        self.state.map.persistence = f64::MAX;
                        self.state.persistence_input_value = self.state.map.persistence.to_string();
                    }
                    else{
                        self.state.map.persistence = str.parse().unwrap();
                        self.state.persistence_input_value = self.state.map.persistence.to_string();
                        if !self.state.persistence_input_value.contains("."){
                            self.state.persistence_input_value.push_str(".0");
                        }
                    }
                }
            }
        }
        // self.state.map.generate_data();
        self.state.update();
        return Command::none();
    }

    fn view(&mut self) -> Element<Message>{
        let sizex_label = Text::new("Width:");
        let sizey_label = Text::new("Height:");
        let seed_label = Text::new("Seed:");
        let freq_label = Text::new("Frequency:");
        let octaves_label = Text::new("Octaves:");
        let lacunarity_label = Text::new("Lacunarity:");
        let persistence_label = Text::new("Persistence:");
        let sizex_input = TextInput::new(&mut self.state.size_x_input_state, "Enter width", &self.state.size_x_input_value, Message::SizeXInputChanged).width(Length::Units(80));
        let sizey_input = TextInput::new(&mut self.state.size_y_input_state, "Enter height", &self.state.size_y_input_value, Message::SizeYInputChanged).width(Length::Units(80));
        let seed_input = TextInput::new(&mut self.state.seed_input_state, "Enter seed", &self.state.seed_input_value, Message::SeedInputChanged).width(Length::Units(80));
        let seedarrup = Button::new(&mut self.state.seedarrupbutton, Text::new("")).on_press(Message::SeedArrUpPressed).height(Length::Units(10)).width(Length::Units(20));
        let seedarrdown = Button::new(&mut self.state.seedarrdownbutton, Text::new("")).on_press(Message::SeedArrDownPressed).height(Length::Units(10)).width(Length::Units(20));
        let seedbcol = Column::new().push(seedarrup).push(seedarrdown);
        let freq_input = TextInput::new(&mut self.state.freq_input_state, "Enter frequency", &self.state.freq_input_value, Message::FreqInputChanged).width(Length::Units(80));
        let freqarrup = Button::new(&mut self.state.freqarrupbutton, Text::new("")).on_press(Message::FreqArrUpPressed).height(Length::Units(10)).width(Length::Units(20));
        let freqarrdown = Button::new(&mut self.state.freqarrdownbutton, Text::new("")).on_press(Message::FreqArrDownPressed).height(Length::Units(10)).width(Length::Units(20));
        let freqbcol = Column::new().push(freqarrup).push(freqarrdown);
        let octaves_input = TextInput::new(&mut self.state.octaves_input_state, "Enter octaves", &self.state.octaves_input_value, Message::OctavesInputChanged).width(Length::Units(80));
        let octavesarrup = Button::new(&mut self.state.octavesarrupbutton, Text::new("")).on_press(Message::OctavesArrUpPressed).height(Length::Units(10)).width(Length::Units(20));
        let octavesarrdown = Button::new(&mut self.state.octavesarrdownbutton, Text::new("")).on_press(Message::OctavesArrDownPressed).height(Length::Units(10)).width(Length::Units(20));
        let octavesbcol = Column::new().push(octavesarrup).push(octavesarrdown);
        let lacunarity_input = TextInput::new(&mut self.state.lacunarity_input_state, "Enter lacunarity", &self.state.lacunarity_input_value, Message::LacunarityInputChanged).width(Length::Units(80));
        let lacunarityarrup = Button::new(&mut self.state.lacunarityarrupbutton, Text::new("")).on_press(Message::LacunarityArrUpPressed).height(Length::Units(10)).width(Length::Units(20));
        let lacunarityarrdown = Button::new(&mut self.state.lacunarityarrdownbutton, Text::new("")).on_press(Message::LacunarityArrDownPressed).height(Length::Units(10)).width(Length::Units(20));
        let lacunaritybcol = Column::new().push(lacunarityarrup).push(lacunarityarrdown);
        let persistence_input = TextInput::new(&mut self.state.persistence_input_state, "Enter persistence", &self.state.persistence_input_value, Message::PersistenceInputChanged).width(Length::Units(80));
        let persistencearrup = Button::new(&mut self.state.persistencearrupbutton, Text::new("")).on_press(Message::PersistenceArrUpPressed).height(Length::Units(10)).width(Length::Units(20));
        let persistencearrdown = Button::new(&mut self.state.persistencearrdownbutton, Text::new("")).on_press(Message::PersistenceArrDownPressed).height(Length::Units(10)).width(Length::Units(20));
        let persistencebcol = Column::new().push(persistencearrup).push(persistencearrdown);
        let rwidth = Row::new().push(sizex_label).push(sizex_input);
        let rheight = Row::new().push(sizey_label).push(sizey_input);
        let rseed = Row::new().push(seed_label).push(seed_input).push(seedbcol);
        let rfreq = Row::new().push(freq_label).push(freq_input).push(freqbcol);
        let roctaves = Row::new().push(octaves_label).push(octaves_input).push(octavesbcol);
        let rlacunarity = Row::new().push(lacunarity_label).push(lacunarity_input).push(lacunaritybcol);
        let rpersistence = Row::new().push(persistence_label).push(persistence_input).push(persistencebcol);
        let canvas = Canvas::new(&mut self.state.map);
        Column::new()
        .push(rwidth)
        .push(rheight)
        .push(rseed)
        .push(rfreq)
        .push(roctaves)
        .push(rlacunarity)
        .push(rpersistence)
        .push(canvas)
        .into()
    }
}

//Check whether a string contains numbers only
fn isnum(str: &str) -> bool{
    let nums = String::from("1234567890.");
    for letter in str.chars(){
        if !nums.contains(&letter.to_string()){
            return false;
        }
    }
    return true;
}

impl Program<Message> for Map<> {
    fn draw(&self, bounds: Rectangle, cursor: canvas::Cursor) -> Vec<Geometry> {
        let map = self.map_cache.draw(Size::new(self.size_x as f32, self.size_y as f32), |frame|{
            let mut fbm = Fbm::new();
            fbm = fbm.set_seed(self.seed);
            fbm = fbm.set_frequency(self.freq);
            fbm = fbm.set_octaves(self.octaves);
            fbm = fbm.set_lacunarity(self.lacunarity);
            fbm = fbm.set_persistence(self.persistence);
            for y in 0..self.size_y{
                for x in 0..self.size_x{
                    let noise = fbm.get([x as f64, y as f64]);
                    if USE_COLOR {
                        let mut pixel:[f32; 3] = [0.0, 0.0, 0.0];
                        if noise < -0.7{
                            pixel = [0.0, 255.0, 255.0];//water
                        }
                        else if -0.7 < noise && noise < -0.6{
                            pixel = [127.0, 51.0, 0.0];//dirt
                        }
                        else if -0.6 < noise && noise < -0.3{
                            pixel = [128.0, 128.0, 128.0];//stone floor
                        }
                        else if -0.3 < noise {
                            pixel = [64.0, 64.0, 64.0];//stone wall
                        }
                        frame.fill_rectangle(Point::new(x as f32, y as f32), Size::new(1.0, 1.0), Color::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0, 1.0))
                    }
                    else{
                        let mut color:f32 = 255.0/2.0;
                        if noise > 0.0{
                            color -= (255.0/2.0)*(noise as f32/1.0);
                        }
                        else{
                            color += (255.0/2.0)*(noise as f32/-1.0);
                        }
                        color = color/255.0;
                        frame.fill_rectangle(Point::new(x as f32, y as f32), Size::new(1.0, 1.0), Color::new(color, color, color, 1.0));
                    }
                }
            }
        });
        vec![map]
    }
}