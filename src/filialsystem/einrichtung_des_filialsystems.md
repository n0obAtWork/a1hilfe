# Einrichtung des Filialsystems

<!-- source: https://amic.de/hilfe/_RepliEinrichtung.htm -->

<p class="just-emphasize">Vorbereitendende Schritte</p>

Identifikationskonzept

Folgende Fragen müssen geklärt werden:

\- Welche Betriebsstätte darf welche Artikelnummern, Kundennummern, Preisgruppen, etc. generieren (z.B. Zentrale)?

\- Wie ist die Primary Key Behandlung der in den Publikationen enthaltenen Publikationsartikel organisiert (z.B. Ident über Ident-Tabelle oder GUID oder sonst wie einzigartig vergebener Schlüssel)?

  - Sind Publikationsartikel vorhanden, die der Einzigartigkeit in der Primary Key-Behandlung widersprechen, so muss dies abgestellt werden. Dies kann durch entfernen des Publikationsartikels aus der Publikation erreicht werden oder es muss eine Änderung der Behandlung vorgenommen werden.

Einrichtung der Filialstruktur **[BST]**

\- [Einrichtung der Betriebsstätten/Filialen](./stammdaten/betriebsstaetten_filialen/index.md)

\- [Ident-Kontingente](./stammdaten/betriebsstaetten_filialen/variante_betriebsstaetten/index.md) für die einzelnen Betriebsstätten/Filialen einrichten

\- [Mandantenstamm](../firmenstamm/firmenkonstanten/mandantenstamm.md#Registerkarte_Replikation)

Einrichtung Stammdaten

\- Einrichtung sämtlicher Stammdaten und steuernder Elemente überarbeiten. Vollständig zentralisierte Einrichtungen sämtlicher Stammdaten insbesondere z.B. auch

  - [Bedienerstamm](../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md)
  - [Bediener](../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md)(neue Bediener anlegen oder [Bediener clonen](../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bediener_clonen.md))
  - [Bedienerklassen](../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md) für neue Filialen einrichten und den Bedienern zuordnen
  - [Formulare](../vorgangsabwicklung/formularzuordnung/formular_formularzuordnungen_zum_vorgang_unterklasse.md)
  - [Nummernkreise](../firmenstamm/nummernkreise_fuer_ware_und_fibu/index.md#Einrichtung_von_Nummernkreisen)
  - [Drucker](../firmenstamm/druckereinrichtung/index.md)
  - [Inventurgruppen](../abschluesse_inventur/inventur/inventurgruppe.md)
  - [Vorgangsunterklassen](../zusatzprogramme/formulareinrichtung_und_zuordnung/formular_importe/vorgangsunterklassen_bearbeiten_sf5.md) (insbes. für Barverkauf)

    vollständig und Betriebsstätten-spezifisch vornehmen.

Artikeleinrichtung

\- Läger und Zuordnungen zu Filialen

\- Artikel für Filialläger anlegen

Vorbereiten der Infrastruktur

Einrichten der Verzeichnisstruktur für den Austausch der SQL Remote Nachrichten und der Log-Dateien, sowie für etwaig benötigte Aufgaben innerhalb der Replikation (Datenbank-Extraktion, FTP, usw.).

Im Aeins-Verzeichnis muss folgendes Verzeichnis erstellt werden:

***..\\Aeins\\dbrexp***

**Folgende Aufgaben sind zusätzlich notwendig, wenn zuvor ein Vorläufer des Filialsystems in Betrieb war:**

Vorarbeiten Filialen

\- Aufträge, Lieferscheine fakturieren

\- Exportfreigaben für Filialexport (ggf. nicht nur Rechnungen und Gutschriften)

\- Filialdaten auslagern

Vorarbeiten Zentrale

\- Eventuelle Importdaten aus Filialsystem alt übernehmen

\- Lieferscheine fakturieren, FiBu-Übertrage, etc.

\- Mandantenserver leerbuchen

\- Filialsystem alt abschalten:

  - Ausführen der SQL-Skripte:
- AMIC_FILIA_NULLSETZER
- AMIC_FILIA_ALT_DEINSTALLIEREN
- Ausführen von 'call AMIC_FILIA_CLEAR' unter OSQL
  - Ggf. Fehlerprotokoll löschen

Update

\- Update der Zentraldatenbank durchführen

Reorganisation

\- Datenbank reorganisieren
