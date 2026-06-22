# Business Intelligence

<!-- source: https://amic.de/hilfe/businessintelligence.htm -->

Als Erweiterung der bisherigen Schnittstelle aller Auswahllisten und Grid Darstellungen zum Excel System (STRG + E) kann jetzt ein Business Intelligence Interface im A.eins genutzt werden, um komplexe Auswertungen auf Basis von A.eins-Daten zu erstellen. Die Grundlage bildet wiederum unser vorhandenes Auswahllistenelement das einem Excel System direkt zur Verfügung gestellt werden kann. Die Excel Arbeitsmappe kann hierbei dann bequem per Datenverbindung auf diese Informationen zugreifen.

Ein Mehrmandanten Zugriff oder ein Mehrtabllen (BI) Zugriff ist dabei auch Problemlos möglich.

Durch Anwahl der technischen Ebene im Auswahllistensystem („ENTW Konfiguration“) kann hierbei pro Variante (Standard wie auch privat) ein Interface erstellt werden. Die Erstellung wird abgewickelt in einem [Zusatzbildschirm](./erstellen_eine_bi_interfaces.md), in dem auch für mehrere Varianten ein Interface gleichzeitig vorbereitet werden kann. Wird mit der Auslieferung schon ein Interface bereitgestellt, so ist diese Funktion nur zur Erneuerung der Basisstrukturen (neue Felder, andere Auswahlkriterien) einzusetzen.

Während der Erstellung eines Interfaces zu dieser Anwendung wird eine View im System angelegt, die die Daten passend und auf Basis eines dynamischen Auswahlbereiches und/oder Profils bereitstellt. Weiterhin werden zwei neue Menüpunkte im System eingerichtet, die den Zugriff auf die Excel Struktur erlauben, einerseits im Hauptmenü unter dem Abschnitt „Informationen“ und in der zugehörigen Anwendung innerhalb der Optionbox. Diese neue Funktion trägt die Variantenbezeichnung gefolgt von den zwei Buchstaben (BI) als Label.

Zusätzlich wird ein Excel Template vorbereitet, welches die Grundlage der späteren Anwendung darstellt.

Nach Erzeugung des Interfaces (bei ggf. auftretenden [Inkompatibilitätsproblemen](./inkompatibilitaetsprobleme.md), sind diese wie [unten](./inkompatibilitaetsprobleme.md) beschrieben zu beheben) kann nach Neustart der A.eins Anwendung sofort mit dem [Excel Blatt](./beispielanwendung_einkauf_verkauf.md) gearbeitet werden. Durch Anwahl der Funktion wird zunächst der Bereichsabfragebildschirm geöffnet, um ggf. einen abweichenden Bereich oder ein anderes Profil auszuwählen. Direkt nach Betätigen der F9 Taste zur Speicherung dieser Bereichsgrenzen wird die Excel Arbeitsmappe geöffnet und durch Betätigen der „[alles Aktualisieren](./aufbau_eines_automatischen_data_refresh_systems.md)“ Funktion werden neue Daten in das Excel Blatt geladen.

Diese geladenen Daten können nun bequem im Excel genutzt werden (drucken, filtern, …) oder es kann eine Überarbeitung dieser Vorlage und anschließender [Speicherung](./rueckspeicherung_von_excel_mappen_mit_geaenderten_einrichtun.md) (im privaten Ordner oder allgemein in der Datenbank) vorgenommen werden.

Während dieser Überarbeitungsphase kann natürlich auch direkt auf andere [BI Bereiche](./aufruf_einer_bi_anwendung.md), die vorher erstellt worden sind, zugegriffen werden. Zusätzlich kann dann im Excel z.B. eine [Pivot Analyse](./pivot_tabellen_erstellen.md) erstellt werden, oder es können Reporte passend zu den in der Firma notwendigen Anforderungen erstellt werden.

Auch eine Automatik ist möglich, die direkt [nach Laden der Excel Mappe](./aufbau_eines_automatischen_data_refresh_systems.md) die Daten in der Excel Mappe aktualisiert.

In jedem Falle ist durch die SHIFT Mausklick Taste ein [Wiedereinspielen](./rueckspeicherung_von_excel_mappen_mit_geaenderten_einrichtun.md) einer veränderten Excel Mappe möglich, um allen Usern diese Mappe zur Verfügung zu stellen, oder diese Mappen in den verschiedenen Mandanten einzurichten.

Um auf einem statischen Bildschirm periodisch Zahlen anzuzeigen ist natürlich auch ein [Datenaktualisierungsintervall](./automatisches_refresh_beim_pivotelement/index.md) einstzellbar.

Auch kann die zugrunde liegende Variante einer Auswahlliste abgeändert werden, um ggf. andere Select-Kriterien zu realisieren oder um einfach noch mehr Felder in die Excel Mappe aufzunehmen. Das [Speichern](./rueckspeicherung_von_excel_mappen_mit_geaenderten_einrichtun.md) einer so vorbereiteten Excel Mappe in einem zentralen CITRIX Applikationsumfeld ist ebenso möglich, wie der Aufruf dieser Mappe aus dem SharePoint und der Office 365 Suite auf einem iPad.

<p class="siehe-auch">Siehe auch:</p>

- [Excel Einrichtung Verbindungen](./excel_einrichtung_verbindungen.md)
- [Hinzufügen von weiteren Feldern auf Basis dieser BI](./hinzufuegen_von_weiteren_feldern_auf_basis_dieser_bi.md)
- [Hinzufügen von Bedingungen auf Basis dieser BI](./hinzufuegen_von_bedingungen_auf_basis_dieser_bi.md)
- [Erstellen eine BI Interfaces](./erstellen_eine_bi_interfaces.md)
- [Aufruf einer BI Anwendung](./aufruf_einer_bi_anwendung.md)
- [Sicherheitsrelevante Einstellungen im EXCEL Umfeld](./sicherheitsrelevante_einstellungen_im_excel_umfeld.md)
- [ODBC Anschluss zur Datenbank](./odbc_anschluss_zur_datenbank.md)
- [Aufbau eines automatischen DATA-Refresh Systems](./aufbau_eines_automatischen_data_refresh_systems.md)
- [Rückspeicherung von Excel Mappen mit geänderten Einrichtungen](./rueckspeicherung_von_excel_mappen_mit_geaenderten_einrichtun.md)
- [Pivot Tabellen erstellen](./pivot_tabellen_erstellen.md)
- [Automatisches Refresh beim Pivotelement.](./automatisches_refresh_beim_pivotelement/index.md)
- [Inkompatibilitätsprobleme](./inkompatibilitaetsprobleme.md)
- [Beispielanwendung Einkauf Verkauf](./beispielanwendung_einkauf_verkauf.md)
