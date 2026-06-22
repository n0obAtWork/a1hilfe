# eRechnung

<!-- source: https://amic.de/hilfe/_erechnung.htm -->

Auf der Registerkarte „eRechnung“ werden alle Daten gepflegt, welche für das gleichnamige Modul gebraucht werden.

**Electronic Address**

Die „Electronic Address“ wird als Information in die XML der eRechnung übernommen. Dabei wird als Mailempfänger die „Electronic Address“ herangezogen, wenn folgende Voraussetzungen erfüllt sind:

1. Der eBeleg ist eingerichtet und der Versandweg generiert eine Mail, in der nur die eRechnung als Anhang enthalten ist.

2. Alle aktuellen Versandprozeduren AMIC_Belegversand_Ware_Spaeter oder AMIC_Belegversand_Ware_Sofort unter [FRZ] auf Tabreiter Abwicklung sind eingetragen.

Hinweis: Private Prozeduren müssen ggf. um dieses Verhalten erweitert werden.

**Leitweg**

Leitweg für die elektronische Zustellung von eRechnung an Behörden.

In der Tabelle kann dem Kunden das passende Profil zugeordnet werden. 

**Versandweg UBL**

Unterschiedliche Möglichkeiten zum Versand von eRechnungen.

| Wert | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 1 | Manuell | Es erfolgt kein automatisierter Versand. Die eRechnung wird über die Varianten in der Belegerfassung generiert. Der Transport der Datei muss anders geregelt werden. |
| 2 | eBeleg ohne PDF | Nur die eRechnung wird per eBeleg verschickt. Es wird kein PDF-Dokument verschickt. Der Empfänger der Mail ist die Adresse, die im Feld „electronic Address“ gepflegt ist. Ist dort nichts gepflegt, wird die Mailadresse über die eBeleg-Einrichtung übermittelt. |
| 3 | eBeleg mit PDF | Dies ist die Vorbelegung. Die eRechnung und das zugehörige PDF-Dokument werden in einer Mail an den Mailempfänger aus der eBeleg-Einrichtung versendet. |
| 4 | eBeleg PDF und XML getrennt | Das PDF und die XML werden in gesonderten Mails verschickt. Die PDF wird an den Mailempfänger gemäß eBeleg-Einrichtung verschickt. Die eRechnung wird an die Mailadresse verschickt, die unter „electronic Address“ eingetragen ist. Ist dort nichts gepflegt, wird auch hier die Mailadresse über die eBeleg-Einrichtung übermittelt. |

Der Versandweg wird nur ausgewertet, wenn das Modul eBeleg für die Belegart eingerichtet ist und eine der beiden Standardprozeduren für den Mailversand (AMIC_Belegversand_Ware_sofort oder AMIC_Belegversand_Ware_später) in [FRZ] unter Abwicklung eingetragen sind. Gibt es dort ältere private Einrichtungen, müssen diese hinsichtlich des oben beschriebenen Verhaltens angepasst werden!

Es bedarf zusätzlicher spezifischer Einstellungen durch den AMIC-Support.

Diese Einstellung wird nicht verwendet beim Exportformat ZugFeRD/CII, die im Profil einstellbar ist. In diesem Fall wird der Versand und die Einbettung des XML im Druck vorgenommen und es gibt keine Notwendigkeit einer Auswahl.

**Profile**

Die eRechnungsprofile, welche dem Kunden zugeordnet werden.

Hierbei kann über die F3-Auswahl aus allen vorhandenen Profilen gewählt werden.

Um ein Profil zu erstellen, muss in [eRechnung Profilpfleger [XRE]](../../erechnung/erechnung_profilpfleger/index.md) ein neues angelegt werden.

Wird hier kein Exportprofil eingetragen, so wird beim Export eines Beleges mit diesem Kunden automatisch das Default-Profil der Vorgangsunterklasse verwendet.

**Importprozeduren Belegfluss**

Für den Import mit anschließender Verarbeitung im Belegfluss können hier Prozeduren zur Informationsbeschaffung eingestellt werden, die die Einstellung des Belegflusspostfachs überschreiben

| Bezeichnung | Bedeutung |
| --- | --- |
| **eRechnung ArtikelInfo** | Diese Prozedur ermittelt anhand einer InvoiceLineId in der eRechnung eine Artikelnummer und eine Artikelbezeichnung.  
Als Standard-Auslieferung und Vorlage für eigene Prozeduren wurde die Demo-Prozedur „AMIC_STD_XRE_ImportLine“ erstellt. |
| **eRechnung ArtikelDetails** | Diese Prozedur ermittelt anhand einer InvoiceLineId in der eRechnung weitere Inforationen zum Abgleich z.B. Mengeneinheiten, Umrechnungsfaktoren etc. und stellt diese den Werten aus der erfassten Quell-Position gegenüber.  
Als Standard-Auslieferung und Vorlage für eigene Prozeduren wurde die Demo-Prozedur „AMIC_STD_XRE_ImportLineDetails“ erstellt. |
