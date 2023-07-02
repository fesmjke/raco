use raylib::prelude::*;
use crate::city::City;

pub struct Preset {
    texture: Texture2D,
    cities: Vec<City>
}

pub enum DefaultPresets {
    USA
}

pub enum PresetSize {
    Small,
    Medium,
    Large
}

impl Preset {
    pub fn new(texture : Texture2D) -> Preset {        
        Self {
            texture,
            cities : vec![]
        }
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn get_cities(&self) -> &Vec<City> {
        &self.cities
    }

    pub fn create_cities(&mut self, input : DefaultPresets, size: PresetSize) {
        match input {
            DefaultPresets::USA => {
                match size {
                    PresetSize::Small => {
                        self.cities = vec![        
                            City::new( 224.7490,  90.3880), // Washington Seattle
                            City::new( 215.0522,  411.2437), // LA California
                            City::new( 609.9511,  429.0715), // Oklahoma City
                            City::new( 940.3322,  580.6557), // Florida Orlando
                            City::new( 377.8781,  151.6298), // Montana Helena
                            City::new( 652.1351,  557.8151), // Texas Houston
                            City::new( 782.6872,  269.3301), // Illinois Chicago
                            City::new( 1022.2527,  260.7585), // New York
                        ]
                    },
                    PresetSize::Medium => {
                        self.cities = vec![        
                            City::new( 224.7490,  90.3880), // Washington Seattle
                            City::new( 330.4484,  440.0740), // Arizona Phoenix
                            City::new( 215.0522,  411.2437), // LA California
                            City::new( 609.9511,  429.0715), // Oklahoma City
                            City::new( 483.7392,  315.9903), // Colorado Denver
                            City::new( 940.3322,  580.6557), // Florida Orlando
                            City::new( 870.7490,  450.3880), // Georgia Atlanta
                            City::new( 377.8781,  151.6298), // Montana Helena
                            City::new( 652.1351,  557.8151), // Texas Houston
                            City::new( 782.6872,  269.3301), // Illinois Chicago
                            City::new( 1022.2527,  260.7585), // New York
                        ]
                    },
                    PresetSize::Large => {
                        self.cities = vec![        
                            City::new( 224.7490,  90.3880), // Washington Seattle
                            City::new( 815.5207,  460.8025), // Alabama Birmingham 
                            City::new( 330.4484,  440.0740), // Arizona Phoenix
                            City::new( 710.7465,  435.2896), // Little Rock Arkansas
                            City::new( 215.0522,  411.2437), // LA California
                            City::new( 609.9511,  429.0715), // Oklahoma City
                            City::new( 483.7392,  315.9903), // Colorado Denver
                            City::new( 1040.7658,  230.6734), // Connecticut Hartford
                            City::new( 984.7391,  417.5398), // North Carolina Wilmington
                            City::new( 940.3322,  580.6557), // Florida Orlando
                            City::new( 870.7490,  450.3880), // Georgia Atlanta
                            City::new( 220.6150,  146.2023), // Oregon Portland 
                            City::new( 377.8781,  151.6298), // Montana Helena
                            City::new( 652.1351,  557.8151), // Texas Houston
                            City::new( 440.3656, 422.3301), // New Mexico Albuquerque
                            City::new( 782.6872,  269.3301), // Illinois Chicago
                            City::new( 1022.2527,  260.7585), // New York
                        ]
                    },
                }
            },
        }
    }
}