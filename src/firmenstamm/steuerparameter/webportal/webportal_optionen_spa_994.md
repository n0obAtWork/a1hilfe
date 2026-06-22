# Webportal-Optionen(SPA 994)

<!-- source: https://amic.de/hilfe/_SPA_994.htm -->

Komplexer Steuerparameter.

Hier können über das Systemformat „WebPortOpt“ verschiedene Optionen für das Webportal angegeben werden.

• **BedienerId ab:** Untere Grenze, ab welcher Nummer Bediener für das Webportal im Bedienerstamm angelegt werden sollen

• **Mailtyp:** Typ unter dem die Mail-Adressen der WebPortal-Kunden im Kundenstamm in der Hauptanschrift für das Portal gespeichert sind. Ist diese Option nicht eingerichtet oder wird unter dem angegebenen Mailtyp keine Mailadresse gefunden, so wird, falls eingerichtet, die unter dem Mailtyp 1 (Standard) eingerichtete Mailadresse gesucht.

• **Versandprofilnummer:** Nummer/Id des Versandprofiles aus dem Versandprofilstamm für Mailversand aus dem Webportal

• **Bestellung Vorgangsklasse:** Vorgangsklasse für die Bestellungen/Bestellanfragen, die aus dem WebPortal heraus in A.eins generiert werden. Ist in dieser Option keine Vorgangsklasse zugeordnet, so wird ein Vorgang der Klasse 400 (Auftrag) erzeugt.

• **Bestellung Unterklasse:** Vorgangsunterklasse für die Bestellungen/Bestellanfragen, die aus dem WebPortal heraus in A.eins generiert werden. Diese Option muss eingerichtet sein.

• **KennwortVergessen-MailBody-Prozedur:** Name einer privaten Prozedur zum ermitteln des Mailbodys für die Kennwortvergessen-Funktion.

• **Dokumente ab Datum:** Ist hier ein Datum angegeben, so werden im WebPortal nur Vorgänge und Dokumente angezeigt, deren Belegdatum größer oder gleich dem angegebenen Datum ist. Ist hier kein Datum angegeben, werden nur Belege des laufenden Kalenderjahres berücksichtigt.

• **Webportal-Archiv-Daten-Filter-Funktion:** Hinterlegen Sie hier eine private Funktion zum Filtern von Archivdaten, die im Webportal angezeigt werden sollen.

• **Webportal-Archiv-Gruppentyp:** Hinterlegen Sie hier den Typ aus dem Anwendungsformat AF_FA_GRUPPE, welcher für die Kennzeichnung von Webportal-Dokumenten angegeben wurde.

• **Rohware-Details-NurErsteWaPos:** Ist für diese Option einer der Werte ‚Ja‘ oder ‚1‘ eingetragen, so erscheinen in der Rohwaren-Detailansicht des WebPortals nur die jeweils ersten Positionen (Lieferposition) der Rohwaren-Vorgänge.

• **Standard-Bestell-Lagernummer:** Lagernummer für zu erzeugende Aufträge bzw. Angebote der WebPortal-Bestellungen, wenn der Artikel nicht per privater Datenbank-Funktion ermittelt werden soll. Zum vom Besteller im WebPortal ausgewählten Artikel(-stamm) wird der zum Artikelstamm passende Artikel des hier angegebenen Lagers ermittelt. Ist hier keine Lagernummer angegeben, so wird als Standard-Bestell-Lager das Lager 0 verwendet.

• **DB-Funktion zur ArtikelId-Bestimmung:** Eine für diese Option eingetragene Datenbank-Funktion soll zum vom Besteller im WebPortal ausgewählten Artikel(-stamm) die ArtikelId für die Erzeugung des Auftragsvorgangs oder Angebotsvorgangs ermitteln. Der Funktionsaufruf erfolgt mit den fünf Parametern *‚ArtiStammId‘* zum gewählten Artikel, *‚KundId‘* des Bestellers, *‚AdressId‘* der vom Besteller gewählten Anschrift, *‚Bestellmenge‘* und *‚Mengeneinheitsnummer‘* zur Bestellmenge.  
Wird keine Datenbank-Funktion angegeben, so erfolgt die Ermittlung des Artikels zu dem in der Option **Standard-Bestell-Lagernummer** angegebenen Lager.

• **Adresstypen für Versandanschriften:** Adresstypen-Liste der Anschrift-Stamm-Einträge des Kunden, die im Portal als Versandanschrift gewählt werden können. Ist für diese Option nichts angegeben, so werden im Standard die Adresstypen 11 (Kundenhauptanschrift) und 12 (Kunden-Versandanschrift) zur Auswahl herangezogen. Zusätzlich können noch Anschriften des Adresstyps 41 (Objektanschriften) angegeben werden. Soll eine Kombination abweichend von der Standardauswahl mit der Option festgelegt werden, so sind alle zu berücksichtigenden Adresstypen durch Komma getrennt anzugeben.  
Beispiel: 12,41 für Kunden-Versandanschriften und Objektanschriften aber keine Kundenhauptanschrift.

• **DB-Prozedur zur Versandanschriften-Text-Aufbereitung:** Private Datenbank-Funktion zum erzeugen des Anzeigetextes aus Anschrift-Daten für die Versandadressen-Auswahl.  
Der Funktionsaufruf erfolgt mit denParametern *‚AdressId‘*, *‚KundId‘*, *‚AdressTyp‘*, *‚Adressnummer‘*, *‚Adressbezeichnung‘* und dem bereits zuvor erzeugten *Standardadresstext‘.*  
Ist mit dieser Option keine DB-Funktion angegeben, so wird der Standardadresstext als Anzeigetext herangezogen.
