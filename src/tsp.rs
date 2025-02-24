// TODO I don't like this solution, so later think about another way of solving this
use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
};

use crate::{city::City, route::Route, Rendereable, Solver};

pub struct TSP {
    pub map: Texture2D,
    pub routes: Vec<Route>,
    pub best_route: Route,
}

impl TSP {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            map: texture,
            routes: vec![],
            best_route: Route::new(vec![]),
        }
    }

    pub fn get_routes<'a, S>(&mut self, cities: &[City]) -> &Vec<Route>
    where
        S: Solver<'a>,
    {
        // TODO add cache like solver: {points:routes}?? idk

        let routes = S::get_routes(cities);
        let best = S::solve(&routes);

        self.routes = routes;
        self.best_route = best;

        &self.routes
    }
}

impl Rendereable for TSP {
    fn render(&self) {
        draw_texture(&self.map, 0., 0., WHITE);
    }
}
