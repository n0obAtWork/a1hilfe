# Schema

<!-- source: https://amic.de/hilfe/_ prodI_Schema.htm -->

Die Struktur des XML beschreibt das nachfolgende Schema:

```xml
<?xml version="1.0" encoding="utf-8"?>
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:import namespace="http://microsoft.com/wsdl/types/" />
  <xs:element name="Produktion" nillable="true" type="Produktion" />
  <xs:complexType name="Produktion">
    <xs:complexContent mixed="false">
      <xs:extension base="ClassExtender">
        <xs:sequence>
          <xs:element minOccurs="0" maxOccurs="1" name="Produkte" type="ArrayOfProdukt" />
          <xs:element minOccurs="0" maxOccurs="1" name="Komponenten" type="ArrayOfKomponente" />
          <xs:element minOccurs="1" maxOccurs="1" name="ProduktionsNummer" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="Jahrnummer" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="VorgangsKlasse" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="VorgangsUnterKlasse" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="VorgangsGuid" xmlns:q1="http://microsoft.com/wsdl/types/" type="q1:guid" />
          <xs:element minOccurs="1" maxOccurs="1" name="PositionsGuid" xmlns:q2="http://microsoft.com/wsdl/types/" type="q2:guid" />
          <xs:element minOccurs="1" maxOccurs="1" name="Status" type="xs:int" />
          <xs:element minOccurs="0" maxOccurs="1" name="Command" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="Linie" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="Produktionstyp" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="Kundenauftragsnummer" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="Ladetraeger" type="ArrayOfLadetraeger" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ClassExtender" />
  <xs:complexType name="Ladetraeger">
    <xs:complexContent mixed="false">
      <xs:extension base="ClassExtender">
        <xs:sequence>
          <xs:element minOccurs="0" maxOccurs="1" name="NVE" type="xs:string" />
          <xs:element minOccurs="1" maxOccurs="1" name="LadetraegerNummer" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="SollMenge" type="xs:decimal" />
          <xs:element minOccurs="1" maxOccurs="1" name="IstMenge" type="xs:decimal" />
          <xs:element minOccurs="1" maxOccurs="1" name="Tara" type="xs:decimal" />
          <xs:element minOccurs="0" maxOccurs="1" name="MengenEinheit" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="LadetraegerTyp" type="xs:string" />
          <xs:element minOccurs="1" maxOccurs="1" name="ReturnTyp" type="ReturnType" />
          <xs:element minOccurs="0" maxOccurs="1" name="Artikel" type="ArrayOfArtikelInfo" />
          <xs:element minOccurs="1" maxOccurs="1" name="Hoehe" type="xs:int" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:simpleType name="ReturnType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="None" />
      <xs:enumeration value="Product" />
      <xs:enumeration value="Component" />
      <xs:enumeration value="Consumption" />
      <xs:enumeration value="Difference" />
    </xs:restriction>
  </xs:simpleType>
  <xs:complexType name="ArrayOfArtikelInfo">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="unbounded" name="ArtikelInfo" nillable="true" type="ArtikelInfo" />
    </xs:sequence>
  </xs:complexType>
  <xs:complexType name="ArtikelInfo">
    <xs:complexContent mixed="false">
      <xs:extension base="ClassExtender">
        <xs:sequence>
          <xs:element minOccurs="1" maxOccurs="1" name="Guid" nillable="true" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="ArtikelNummer" type="xs:string" />
          <xs:element minOccurs="1" maxOccurs="1" name="LagerNummer" nillable="true" type="xs:int" />
          <xs:element minOccurs="0" maxOccurs="1" name="ArtikelBezeichnung" type="xs:string" />
          <xs:element minOccurs="1" maxOccurs="1" name="Menge" type="xs:decimal" />
          <xs:element minOccurs="0" maxOccurs="1" name="GebindeAnzahl" nillable="true" type="xs:decimal" />
          <xs:element minOccurs="0" maxOccurs="1" name="MengenEinheit" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="Partien" type="ArrayOfPartie" />
          <xs:element minOccurs="0" maxOccurs="1" name="OriginalArtikelNummer" type="xs:string" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ArrayOfPartie">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="unbounded" name="Partie" nillable="true" type="Partie" />
    </xs:sequence>
  </xs:complexType>
  <xs:complexType name="Partie">
    <xs:complexContent mixed="false">
      <xs:extension base="ClassExtender">
        <xs:sequence>
          <xs:element minOccurs="0" maxOccurs="1" name="Partiebezeichnung" type="xs:string" />
          <xs:element minOccurs="1" maxOccurs="1" name="Partienummer" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="PartieMenge" type="xs:decimal" />
          <xs:element minOccurs="0" maxOccurs="1" name="MengenEinheit" type="xs:string" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="Komponente">
    <xs:complexContent mixed="false">
      <xs:extension base="ArtikelInfo">
        <xs:sequence>
          <xs:element minOccurs="0" maxOccurs="1" name="Typ" type="xs:string" />
          <xs:element minOccurs="0" maxOccurs="1" name="Spareparts" type="ArrayOfArtikelInfo" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="Produkt">
    <xs:complexContent mixed="false">
      <xs:extension base="ArtikelInfo">
        <xs:sequence>
          <xs:element minOccurs="0" maxOccurs="1" name="Typ" type="xs:string" />
          <xs:element minOccurs="1" maxOccurs="1" name="RezepturGruppe" type="xs:int" />
          <xs:element minOccurs="1" maxOccurs="1" name="RezepturVariante" type="xs:int" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ArrayOfProdukt">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="unbounded" name="Produkt" nillable="true" type="Produkt" />
    </xs:sequence>
  </xs:complexType>
  <xs:complexType name="ArrayOfKomponente">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="unbounded" name="Komponente" nillable="true" type="Komponente" />
    </xs:sequence>
  </xs:complexType>
  <xs:complexType name="ArrayOfLadetraeger">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="unbounded" name="Ladetraeger" nillable="true" type="Ladetraeger" />
    </xs:sequence>
  </xs:complexType>
</xs:schema>
```
