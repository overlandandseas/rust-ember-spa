import Ember from 'ember';
import ENV from 'todo-rust/config/environment';

export default Ember.Route.extend({
  init() {
    console.log("Enviroment", ENV.routePrefix);
  },

  setupController(controller) {
    this._super(...arguments);
    console.log('Calling', `${ENV.routePrefix}api/games`);
    fetch(`${ENV.routePrefix}api/games`, {
      header: {
        'Access-Control-Allow-Origin': '*',
        'Content-Type': 'application/json'
      },
    }).then(resp => resp.json())
    .then(data => {
      controller.set('games', data.games);
    })
  }
});
