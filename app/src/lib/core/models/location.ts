export class Location {
  public type: string;
  public municipality: string;
  public street: string;
  public houseNumber: string;
  public displayName: string;
  public longitude: number;
  public latitude: number;

  constructor(type: string, municipality: string, street: string, houseNumber: string, displayName: string, longitude: number, latitude: number) {
    this.type = type;
    this.municipality = municipality;
    this.street = street;
    this.houseNumber = houseNumber;
    this.displayName = displayName;
    this.longitude = longitude;
    this.latitude = latitude;
  }

  public static fromJSON(json:
    {
      type: string,
      gemeentenaam: string;
      straatnaam: string;
      huis_nlt: string;
      weergavenaam: string;
      centroide_ll: {x: number, y: number}
    }): Location {
    return new Location(json.type, json.gemeentenaam, json.straatnaam, json.huis_nlt, json.weergavenaam, json.centroide_ll.x, json.centroide_ll.y);
  }

    public static fromStore(json:
    {
      type: string,
      municipality: string;
      street: string;
      houseNumber: string;
      displayName: string;
      longitude: number;
      latitude: number;
    }): Location {
    return new Location(json.type, json.municipality, json.street, json.houseNumber, json.displayName, json.longitude, json.latitude);
  }
}
