export class Location {
  public municipality: string;
  public street: string;
  public displayName: string;
  public longitude: number;
  public latitude: number;

  constructor(municipality: string, street: string, displayName: string, longitude: number, latitude: number) {
    this.municipality = municipality;
    this.street = street;
    this.displayName = displayName;
    this.longitude = longitude;
    this.latitude = latitude;
  }

    public static fromJSON(json: { gemeentenaam: string; straatnaam: string; weergavenaam: string; centroide_ll: {x: number, y: number} }): Location {
    return new Location(json.gemeentenaam, json.straatnaam, json.weergavenaam, json.centroide_ll.x, json.centroide_ll.y);
  }
}
