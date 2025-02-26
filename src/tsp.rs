// TODO I don't like this solution, so later think about another way of solving this

use crate::{city::City, route::Route, utils::worst_route, Solver};

pub struct TSP {
    pub routes: Vec<Route>,
    pub best_route: Route,
    pub worst_route: Route,
}

impl TSP {
    pub fn new() -> Self {
        Self {
            routes: vec![],
            best_route: Route::new(vec![]),
            worst_route: Route::new(vec![]),
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

        self.worst_route = worst_route(&self.routes);

        &self.routes
    }
}

// impl Rendereable for TSP {
//     fn render(&self) {
//         draw_texture(&self.map, 0., 0., WHITE);
//     }
// }
