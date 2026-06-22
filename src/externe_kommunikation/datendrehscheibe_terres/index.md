# Datendrehscheibe (TERRES)

<!-- source: https://amic.de/hilfe/_terres_main.htm -->

Hauptmenü > Externe Kommunikation > Datendrehscheibe [TERRE]

Informationen

Aus dem Terres System können Artikel, Preise, Lieferanten und Buchungsdaten in A.eins eingespielt werden. Die von Terres bereitgestellten Daten werden auf einem Download Server bereitgestellt. Der Datenaustausch findet bei Terres über XML statt. Die Struktur der Dateien ist anhand dieser XML Datei ersichtlich.

Bei der Ersteinrichtung eines Marktes werden ca. 300000 Artikel nebst Preisen und Lieferanten in XML bereitgestellt und in das A.eins System eingespielt.

Importieren der Daten in das A.eins System

Sind alle von Terres stammende XML Dateien mit den Artikeln und Preisen von dem Download Server kopiert worden, können diese in das A.eins System eingespielt werden. Es gibt zwei Wege wie die Terres Dateien in das System A.eins eingespielt werden können. Durch einen [Automatisch](./index.md#Nachtlauf)en Nachtlauf welcher per Event gesteuert wird, oder durch einen manuellen Lauf der per Hand ausgelöst wird.

Bevor die XML Datei in das A.eins System eingespielt werden, müssen noch einige Einstellungen berücksichtig werden.

Hauptmenü > Externe Kommunikation \> Datendrehschreibe > Artikelimportverfahren Direktsprung **[TERRE]**

In dieser Auswahlliste sind nach einer erfolgreichen Einspielung die Terres Artikel zu sehen.

Um den Import zu starten wird die Funktion Datendrehscheibe [F9] aufgerufen. Wird das Anzeigefenster zum ersten Mal geöffnet, so öffnet sich ein Dialog in dem das Verzeichnis ausgewählt wird, in dem sich die zu importierenden Daten befinden. Es muss dabei beachtet werden, dass das Verzeichnis sich relative zum Datenbankserver befindet. Dieses Verzeichnis kann jederzeit geändert werden.

Auch wenn das Importieren der Daten per Event läuft ist darauf zu achten, dass das Verzeichnis angegeben wird.

Nach dem der Pfad richtig eingetragen worden ist, existiert noch die Möglichkeit ein Trace einzuschalten. Das Trace protokolliert wie viele Datensätze bei dem Import eingespielt worden sind. Des Weiteren wird beim Artikelmimport mitprotokoliert wie viel Zeit für welche Funktion benötigt worden ist. In der Funktion AMIC_ARTIKELEIN_INSUPD wird protokolliert wie viele Artikel pro fünf Minuten eingespielt worden sind.

Das Einspielen wird über die Funktion Daten einspielen [F8] gestartet.

Wird der Import ausgeführt, so wird versucht alle XML-Dateien, die sich auf diesem Verzeichnis befinden, in den ArtikelStammTerres einzuspielen.

Wenn die Daten korrekt eingelesen werden konnten, werden die Dateien in „\*.XML_IMPORTED“ umbenannt. Hat der Import nicht funktioniert, so werden die Dateien in „\*.XML_ERROR“ umbenannt.

Die eigentliche Verarbeitung der XML-Dateien erfolgt durch eine Funktion mit dem Namen „TERRES_IMPORT“. Diese hat keine Parameter und liefert als Ergebnis die Anzahl der eingelesenen Datensätze oder eine Zahl kleiner 0, wenn ein Fehler aufgetreten ist.

Soll der Nachtlauf per Event passieren, sollte das Event folgenden Struktur besitzen.

Der Artikelimport wurde so entwickelt, dass er auch automatisiert über ein Event gesteuert werden kann. Sind die entsprechenden Optionen - Datei-Verzeichnis, privatisierbare Funktionen, Lagerzuordnung sowie die Importumsetzer Schlüsselklassen – eingerichtet, kann man sich ein Event einrichten, dass die Daten täglich aktualisiert. Dazu richtet man sich einen neues [Event](../../zusatzprogramme/client_cache/index.md) (Direktsprung [**EVT]**) ein. Auf dem Register Vorlagen kann man für die Datendrehscheibe eine Vorlage aktivieren, die den Dateiimport und den Artikelimport übernimmt Der Parameter „in_ArtikelImport“ in der Prozedur „amic_evt_datendrehscheibe“ kann die Ausprägung 0 oder 1 annehmen. Bei 0 werden die Artikel nicht in das A.eins System übernommen, die Daten werden nur in ArtikelStammTerres eingespielt. Bei 1 werden die Daten auch gleich in das A.eins System komplett eingespielt.

Sollen die Daten komplett eingespielt werden, so müssten alle Umschlüsselungen, die noch nicht vorhanden sind, von Hand in den Artikel und Artikelstamm übernommen werden. Im Standard wird der Parameter „in_ArtikelImport“ auf 0 gesetzt.

```sql
begin
  call
Fehlerprotokoll(in_text = 'Start
Drehscheibe');
  call
amic_evt_datendrehscheibe(in_ArtikelImport = 0);
  call
Fehlerprotokoll(in_text = 'End
Datendrehscheibe')
  exception
    when
others then
      call Fehlerprotokoll(in_text = 'Abbruch Datendrehscheibe')
end
```

Auswahlliste und Bedeutung einzelner Felder

Nachdem die Dateien eingespielt worden sind, kennzeichnet die zugehörige Auswahlliste die Situation der Gesamtübernahme. Hierbei werden die noch nicht übernommenen Datensätze (Terres Artikel in das A.eins System) in Gelb gekennzeichnet. Alle noch nicht korrekt zugeordneten Umschlüsselwerke werden in rot markiert. Des Weiteren werden ausgelistete Artikel rot markiert. Ablaufende Artikel werden gelb markiert. Es existiert eine Filter Möglichkeit auf der Auswahlliste.

Querverweis zu Terresspezifische Ausprägungen des Terres-Artikels wäre schön

Bevor die eigentliche Artikeldatenübernahme angestoßen wird (auch automatisch möglich), müssen aus dieser Auswahlliste heraus die rot markierten Datensätze im Umschlüsselwerk [bearbeitet](../importumsetzer.md) werden. Die gelb markierten Datensätze zeigen an, dass noch nicht das [Importverfahren](./index.md#ueb_bereich_importumsetzer) in die A.eins Stammsätzen durchgelaufen ist.

Funktionen in der Auswahlliste

| ![\*](../../ImagesExt/image8_1555.jpg "*") Funktion | ![\*](../../ImagesExt/image8_1555.jpg "*") Funktionstaste | ![\*](../../ImagesExt/image8_1555.jpg "*") Bedeutung |
| --- | --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Datendrehscheibe | ![\*](../../ImagesExt/image8_1556.jpg "*") F9 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit der Funktion wird die Maske für die Einstellungen geöffnet. Des Weiteren kann von der Maske aus, das Import Verfahren manuell gestartet werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") VK-Preis anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") F5 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion können die Informationen zu dem Terres Preis angezeigt werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikeleinheiten anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") F6 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion können die Informationen zu der Artikeleinheit angezeigt werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikellieferant anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") SF6 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion können die Informationen zu dem Artikellieferant angezeigt werden. Es können mehrere Lieferanten zu einem Artikel existieren. Der Hauptlieferant wird mit einer 1 gekennzeichnet. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikelreferenz anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") SF5 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion werden die Terres Informationen bezüglich des EAN dargestellt. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikel-Marktbereich anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") CF5 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion können die Informationen zu dem Marktbereich dargestellt werden. Es können mehrere Marktbereiche zu einem Artikel existieren. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Gefahrguttext anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion können die Informationen zu dem Gefahrgut dargestellt werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") A.eins Artikelstamm | ![\*](../../ImagesExt/image8_1556.jpg "*") SF9 | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion wird der A.eins Artikelstamm zu einem Terres Artikel aufgerufen. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Basisstruktur anzeigen | ![\*](../../ImagesExt/image8_1556.jpg "*") | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit dieser Funktion kann die komplette XML des ausgewählten Terres Artikel im Browser angeschaut werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Fehlerprotokoll | ![\*](../../ImagesExt/image8_1556.jpg "*") CF9 | ![\*](../../ImagesExt/image8_1556.jpg "*") Diese Funktion ruft das Fehlerprotokoll auf. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Event Manager | ![\*](../../ImagesExt/image8_1556.jpg "*") SCF9 | ![\*](../../ImagesExt/image8_1556.jpg "*") Diese Funktion ruft den Event Manager auf. |

Beschreibung einzelner Felder der Auswahlliste

Es werden nur einzelne Felder beschrieben. Die Auswahlliste zeigt die wichtigen Informationen aus dem Terres System und den A.eins Kennzeichen an. Die Kennzeichen, die aus dem Terres System stammen, fangen immer mit einem „T-“ an. Die umgeschlüsselten Werte aus dem A.eins System fangen immer mit „A-“ an.

| ![\*](../../ImagesExt/image8_1557.jpg "*") Felder | ![\*](../../ImagesExt/image8_1557.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Stammstatus | ![\*](../../ImagesExt/image8_1556.jpg "*") Das Kennzeichen wird auf 1 gesetzt, wenn ein Artikel neu ist, oder wenn Änderungen am Stammsatz seitens Terres vorgenommen worden sind. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Preisstatus | ![\*](../../ImagesExt/image8_1556.jpg "*") Das Kennzeichen wird auf 1 gesetzt, wenn Preisänderungen zu diesem Artikel existieren. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Etikett | ![\*](../../ImagesExt/image8_1556.jpg "*") Das Kennzeichen wird auf 1 gesetzt, wenn neue Etiketten gedruckt werden sollen. Nach erfolgreichem Etikettendruck muss das Feld auf 0 zurückgesetzt werden. |

Funktion Artikelimport F9

Tatsächliche Übernahme der Daten in den A.einsArtikelstamm.

Bevor der Artikelimport das erste Mal gestartet werden soll, müssen folgende Einstellungen vorgenommen werden. Auf der Auswahlliste wird die Funktion Datendrehscheibe [F9] ausgewählt. Nachdem sich das Anzeigefenster geöffnet hat, wechseln wir auf die Registerkarte Optionen. Bei der Ersteinrichtung sollte die Funktion Vorlage-Terres [SF8] ausgewählt werden. Es werden alle signifikanten Einstellungen mit dem [A.eins Standard](./index.md#ueb_bereich_importumsetzer) vorbelegt. In dem Einrichterparameter des Anzeigefensters kann eine private Prozedur hinterlegt werden, welche die Einspielung der Daten vom ArtikelStammTerres in das A.eins System vornimmt. Diese private Prozedur wird anstelle der [Standardprozedur](./privatisierbare_prozeduren_fuer_die_datendrehscheibe_import.md#proc_ArtikelImport) ausgeführt.

Dabei ist zu beachten, dass alle Prozeduren, die in der Standard Prozedur aufgerufen werden, auch in der privaten Prozedur berücksichtig werden. Ansonsten können wird nicht gewährleisten, dass die Daten komplett richtig eingespielt werden. Unter dem Punkt Privatisierbare Prozeduren für die Datendrehscheibe sind die

Die Prozeduren können privatisiert werden um spezifische Anforderungen eines Marktes zu Implementieren. Die Funktionen besitzen keine Übergabe Parameter.

Mit der Einstellung „Neue Preise gültig in … Tagen“, kann eingerichtet werden, in wie viel Tagen die neuen Preise gelten sollen. Somit besteht die Möglichkeit neue Etiketten rechtzeitig zu drucken und die Artikel im Markt auszuzeichnen.

Lagerzuordnung

In der Lagerzuordnung können mehrere Läger eingetragen werden. Dies bedeutet jedes Lager kann eine andere Einstellung haben.

| ![\*](../../ImagesExt/image8_1557.jpg "*") Feld | ![\*](../../ImagesExt/image8_1557.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lagernummer | ![\*](../../ImagesExt/image8_1556.jpg "*") Für welches Lager sollen die Terresartikel und Preise eingespielt werden. Das Lager kann per F3 ausgewählt werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lagername | ![\*](../../ImagesExt/image8_1556.jpg "*") Bezeichnung des Lagers |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Aktivkennzeichen | ![\*](../../ImagesExt/image8_1556.jpg "*") Über diese Kennzeichen kann gesteuert werden, ob das Lager Aktiv oder Passiv ist. Es werden die Daten in das Lager nur übernommen, wenn die Einstellung auf aktiv steht. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Übernahme Artikel | ![\*](../../ImagesExt/image8_1556.jpg "*") Mit diesem Kennzeichen kann eingestellt werden, ob auf dem Lager die Artikel bei einer Einspielung nur aktualisiert werden sollen, oder ob auch neue Artikel in das Lager eingespielt werden dürfen. Der Standard ist Einspielung und Aktualisierung. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") VK-Nettopreisliste | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier wird die Nettopreisliste hinterlegt, welche für den Terrespreis und dem Lager gelten soll. Die dazugehörige Preismatrix wird im Musterartikel hinterlegt. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") VK-Bruttopreisliste | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier wird die Bruttopreisliste hinterlegt, welche für den Terrespreis und dem Lager gelten soll. Die dazugehörige Preismatrix wird im Musterartikel hinterlegt. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") EK-Preisliste | ![\*](../../ImagesExt/image8_1556.jpg "*") Hier wird die Einkaufspreisliste hinterlegt. Die dazugehörige Preismatrix wird im Musterartikel hinterlegt. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Musterartikel | ![\*](../../ImagesExt/image8_1556.jpg "*") Musterartikel welcher bei der Einspielung berücksichtig werden soll. Dieser muss eingerichtet werden, da vor dem Übernehmen des Artikels ein Musterabgleich durchgeführt wird.  
![\*](../../ImagesExt/image8_1556.jpg "*") Es ist auf jeden Fall darauf zu achten, dass im Musterartikel die richtige Preismatrix zu der jeweiligen Preisliste eingetragen worden ist. Ansonsten können die Preise den Artikeln nicht ordnungsgemäß zugeordnet werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Artikel-Bezeichnung | ![\*](../../ImagesExt/image8_1556.jpg "*") Bezeichnung des Musterartikels |

Bereich Importumsetzer

Der Bereich Importumsetzer ist eigentlich das Herzstück der Terres Schnittstelle.In diesem Umschlüsselwerkwerden den Terres spezifischen Kennzeichen wie z.B. Warengruppe der Warengruppe in A.eins zugeordnet.

Wird der Standard ausgewählt, so wird die Schlüsselklasse für jedes Kennzeichen, welches Umgeschlüsselt werden soll, vorbelegt. Es besteht die Möglichkeit dies zu privatisieren.

Im Importumsetzer in der Variante „Import-Umsetzer Itemboxzuordnung“.

Es existieren bislang sechs Kennzeichen die Umgeschlüsselt werden.

| ![\*](../../ImagesExt/image8_1558.jpg "*") Schlüsselklasse | ![\*](../../ImagesExt/image8_1558.jpg "*") Nummer | ![\*](../../ImagesExt/image8_1558.jpg "*") Terres-Kennzeichen |
| --- | --- | --- |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Warengruppe | ![\*](../../ImagesExt/image8_1556.jpg "*") 1001 | ![\*](../../ImagesExt/image8_1556.jpg "*") Die Warengruppe setzt sich aus folgenden Feldern zusammen. Aus dem GruppenCode, Untergruppencode und der Obergruppencode |
| ![\*](../../ImagesExt/image8_1556.jpg "*") EKZ | ![\*](../../ImagesExt/image8_1556.jpg "*") 1004 | ![\*](../../ImagesExt/image8_1556.jpg "*") Die Erlöskennziffer ist die Produktbuchungsgruppe |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Steuerschlüssel | ![\*](../../ImagesExt/image8_1556.jpg "*") 1005 | ![\*](../../ImagesExt/image8_1556.jpg "*") Der Steuerschlüssel ist die Produktbuchungsgruppe |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Mengeinheit | ![\*](../../ImagesExt/image8_1556.jpg "*") 1002 | ![\*](../../ImagesExt/image8_1556.jpg "*") Bei der Mengeinheit muss der Basiseinheitencode in eine A.eins konforme Mengeinheit umgeschlüsselt werden. Terres kennt nur die Bezeichnung wie z.B. kg. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Mengeinheitsgruppe | ![\*](../../ImagesExt/image8_1556.jpg "*") 1003 | ![\*](../../ImagesExt/image8_1556.jpg "*") Anhand der Mengeinheit muss der Basiseinheitencode in die Mengeinheitsgruppe umgeschlüsselt werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lieferanten | ![\*](../../ImagesExt/image8_1556.jpg "*") 1006 | ![\*](../../ImagesExt/image8_1556.jpg "*") Die Kreditorennr. muss in ein A.eins Lieferant umgewandelt/zugeordnet werden. |
| ![\*](../../ImagesExt/image8_1556.jpg "*") Lager | ![\*](../../ImagesExt/image8_1556.jpg "*") 1007 | ![\*](../../ImagesExt/image8_1556.jpg "*") Umschlüsselung des Lagerortcodes einer Terres Rechnung in eine A.eins Lagernummer. |

Bevor die Artikel übernommen werden können, müssen die Umschlüsselungen vorgenommen werden. Bei der ersten Einrichtung des Marktes empfehlen wir die Tabellarischeansicht zu verwenden. Dazu werden auf der Ansichtsmaske die einzelnen Punkte aufgerufen. Sie können aber auch im Importumsetzer [IMPUM] in der Variante „Import-Umsetzer“ „Ändern(Tabellarisch)“ [F5] aufrufen. Es ist dabei zu beachten, dass bei der Ersteinrichtung das Aufrufen der tabellarischen Ansicht etwas länger dauert. Bei nachträglichen Einspielungen, die neue Kennzeichen enthalten, lassen sich im Importumsetzer die neuen Kennzeichen einfach nachpflegen.

Was muss noch beachtet werden vor dem Artikelimport

Im Artikelstamm auf der Registerkarte [Markt](../../artikelstamm_und_artikel/parameter_des_artikelstamms/registerkarte_markt.md) existiert die Möglichkeit Einstellungen vorzunehmen, dass ein vorhandener Artikel oder Ausprägungen des Artikels nicht von Terres abgeändert werden darf.

Nach dem die Ümschlüsselung abgeschlossen ist, die Musterartikel eingerichtet und eventuell die Kennzeichen im Artikelstamm gesetzt worden sind, können die Terres Daten mit „Artikelimport“ [F9] nun in das A.eins System übernommen werden.

Terresspezifische Ausprägungen des Terres-Artikels

Preise

Der Preise steuert, ob ein Artikel ausläuft oder abgekündigt wird.

1. Wenn die letzte Stelle im Preis eine sieben hat so läuft der Artikel aus und kann nicht mehr über Terres bezogen werden.

2. Wenn ein Artikel den Preis 9999 bekommen hat, ist dieser abgekündigt worden. Des Weiteren wird der Preis von 9999 nicht mehr in das A.eins System übernommen, und das Kennzeichen „Preispflege durch Datendrehscheibe“ wird auf der Registerkarte [Markt](../../artikelstamm_und_artikel/parameter_des_artikelstamms/registerkarte_markt.md) im Artikelstamm auf „unterdrücken“ gesetzt.

Preiseinheiten

Die Preiseinheit und der Preiseinheitscode eines Artikels werden beim Artikelimport in den A.eins Artikelstamm eingetragen. Diese werden auf der Registerkarte Markt im Artikelstamm angezeigt. Die Preiseinheit hat zwölf stellen. Der Preiseinheitscode wird als Text gespeichert z.B. Kg

Der Grundpreis eines Artikels wird wie folgt berechnet:

Preiseinheit \* Preis des Artikels

Diese Informationen können auf dem Etikett mit angedruckt werden.

„Gebinde“

Ein Gebinde Artikel hat in dem Verkauf-Einheitencode der XML Struktur das Kennzeichen GBD. Ist dieses Kennzeichen gesetzt, so wird in die Preiseinheit des Artikels die „Menge-pro-Einheit“ aus dem Knoten „Std-Artikeleinheit“ mit dem „Code“ GBD übernommen. Die Grundpreiseinheit wird als Mengeneinheit des Artikelpreises genommen.

Die Grundpreisauszeichnung für das Etikett wird beim Gebinde Artikel berechnet.

1 / Menge des Verkauf-Einheitencode \* Menge der Grundpreiseinheit.

<p class="siehe-auch">Siehe auch:</p>

- [Privatisierbare Prozeduren für die Datendrehscheibe Import](./privatisierbare_prozeduren_fuer_die_datendrehscheibe_import.md)
- [Bestellung von Artikeln per Datendrehscheibe](./bestellung_von_artikeln_per_datendrehscheibe.md)
- [Belegimport](./belegimport/index.md)
- [Datendrehscheibe Statistik Export](./datendrehscheibe_statistik_export.md)
