import { Location } from "$lib/core/models/location";

export class LocationManager {
  public location = $state<Location>();

  constructor() {}

  public setLocation(location: Location) {
    this.location = location;
  }
}
