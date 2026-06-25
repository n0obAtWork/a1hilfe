# DTA

<!-- source: https://amic.de/hilfe/_dta.htm -->

Hauptmenü > Mahn-,Zahl-, Zinswesen > Zahlungsverkehr > Zahlungen bearbeiten

Direktsprung **[ZHB]**

Die Daten werden für den Datenträgeraustausch aufbereitet. Spezielle Einstellungen lassen sich am [Steuerparameter 919](../../../firmenstamm/steuerparameter/optionen_finanzwesen/dta_optionen_spa_919.md) ändern.

Es stehen mehrere DTA-Verfahren zur Verfügung.  
    

- DTA-Verfahren zwischen Kunde und Bank. Hier wird je nach Einstellung des Steuerparameters (SPA) „DTA-Ausgabeformat“ unterschieden, welches Format für die Dateiausgabe verwendet wird. Zurzeit werden folgende Formate unterstützt:

  1. Deutschland: Datenträgeraustausch zwischen Kunde und Bank.

  2. Österreich: EDIFACT-TRANSAKTION für den Inlandszahlungsverkehr

  3. Dänemark: Unitel for PC Format.

  4. SEPA

  5. Datenbankfunktion: Offenes Format, bei dem eine Datenbankfunktion die Ausgabedaten als long varchar liefert. Diese private Datenbankfunktion wird unter der Option (Direktsprung [OPT]) DTA_PROZEDUR eingetragen. Als Parameter erhält diese Prozedur die ASATZ_ID über die die Aufbereiteten Datensätze identifiziert werden können  
<strong>Achtung:</strong> Wird die Prozedur im Vieraugenprinzip-Zahlung verwendet, so wird die ZahllaufId zur Identifizierung übergeben.  
    

- DTINT Verfahren
- DTA-Verfahren für den Auslandszahlungsverkehr. Der Auslandszahlungsverkehr wird für die DTA-Formate in Deutschland und Dänemark – dort natürlich für alle Länder - unterstützt.
- [SEPA](../sepa/index.md).

Für die drei Formate DTA, DTINT und DTA für Auslandszahlungen sind unterschiedliche Dateinamen vorgesehen, die auch getrennt voneinander abgespeichert werden. Es ist somit auch möglich, diese Verfahren parallel zu nutzen und die Ausgabedateien auf unterschiedliche Verzeichnisse auszugeben.

Um sicherzustellen, dass der an die Bank gesendete Datenträger Fehlerfrei ist, werden diverse Prüfungen vorgenommen:

- In und Auslandszahlungsverkehr darf nicht gemischt sein.
- Nur eine Hausbank pro DTA.
- Nur Zahlungsausgang oder Zahlungseingang pro DTA.
- Nur eine Währung beim DTA für Inlandszahlungsverkehr.
- Nur nicht bereits verarbeitete (gedruckte oder versendete) Zahlungen.
- Hausbankkonto darf nicht mehr als 10 (bzw. 11 in Österreich) Stellen haben.
- Auftraggeber muss im Hausbankenstamm hinterlegt sein.
- Im Inlandszahlungsverkehr ist nur Euro zugelassen.  
    

Hat man unter „[Zahlungsvorschläge erstellen](../zahlungsvorschlaege_erstellen.md)“ in den Einrichterparametern eine Datenbankfunktion zur Bestimmung der Kundenbank hinterlegt, so findet vor dem DTA ein weiterer Testlauf statt, der mit Hilfe dieser Funktion noch einmal die Bank bestimmt und diese mit der Bank im Zahlungsbeleg vergleicht. Weichen diese Banken voneinander ab, so wird ein Fehlerhinweis ausgegeben und dieser Zahlungsbeleg nicht weiter verarbeitet.

Schlägt eine dieser Prüfungen fehl, wird keine Datenträgerdatei erstellt. Bei folgenden Prüfungen wird nur die betroffene Zahlung nicht ausgeführt.

- Bankkonto des Kunden/Lieferanten darf nicht mehr als 10 (bzw. 11 in Österreich) Stellen haben.
- Zahlsperre für diesen Kunden darf nicht gesetzt sein.
- Bankleitzahl der Bank des Kunden/Lieferanten darf nicht mehr als 8(bzw. 5 in Österreich) Stellen haben.  
    

Beim Auslandszahlungsverkehr kommen noch zusätzliche Prüfungen hinzu. Genauere Hinweise befinden sich im Teil [Auslandszahlungsverkehr](../auslandzahlungsverkehr_in_a_eins/index.md).

Sämtliche aufgetretenen Probleme werden ausgegeben und können dann behoben werden. Gegebenenfalls kann dann nach zurücksetzen des Druckkennzeichens der Datenträgeraustausch wiederholt werden.

    
Nach erfolgreicher Erstellung der Ausgabedatei kann eine Banksammelliste und ein Begleitzettel sowie Avisen gedruckt werden. Es ist auch möglich, dass die Avis direkt als [Mail versende](./avis_als_mail_versenden.md)t wird.

Die Funktion „***Ausführungsdatum ändern***“ **F4** steht für die DTA-Verfahren des Inlandszahlungsverkehrs in Deutschland und der über eine Datenbankfunktion gesteuerten Dateierstellung zur Verfügung. Bei der Änderung des Datums werden die Einschränkungen - nicht vor dem Erstelldatum und maximal 15 Tage nach dem Erstelldatum – geprüft. Außerdem werden Samstage und Sonntage als Ausführdatum nicht zugelassen. Bei DTA-Zahlungen im SEPA-Format steht diese Funktion nicht zur Verfügung. Hier kann das Ausführdatum pro [Zahlungsbeleg](./index.md) geändert werden.

Es existieren zum DTA folgende Einrichterparameter:

- Referenznummernkreis:  
Ist hier ein Nummernkreis hinterlegt, so wird für das *DTA Verfahren zwischen Kunde und Bank* hier raus eine Referenznummer generiert, die in der DTA – Datei mit übermittelt wird. Bei Rückfragen bezieht sich die Bank ggf. auf diese Nummer. Diese ist dann im DTA-Archiv zu finden.  
    

- Anzahl Zeilen Verwendungszweck:  
Im *DTA Verfahren zwischen Kunde und Bank* sind bis zu 15 Zeilen Verwendungszweck erlaubt. Einige Übertragungsprogramme erlauben aber weniger Zeilen bzw. ignorieren die meisten Zeilen. Deswegen kann hier eingestellt werden, wie viel Zeilen generiert werden sollen.  
    

- VWZ auf eine Zeile komprimieren:  
Im Normalfall werden pro Beleg zwei Zeilen verwendet und dort Belegnummer, Datum, Betrag und Referenznummer übergeben. Steht dieser Parameter auf **Ja**, dann wird nur Referenznummer (bei Bankeinzug die Belegnummern), Datum und Betrag ausgegeben.  
    

- Belegnummer rechtsbündig ausgeben (gegebenenfalls links abschneiden):  
Belegnummern in A.eins können bis zu 20 Stellen haben. Da der Platz im Verwendungszweck sehr begrenzt ist, werden nur die ersten 8 Stellen davon ausgegeben. Wird dieser Parameter auf **Ja** gesetzt, dann werden die hinteren 8 Stellen ausgegeben.  
    

- Prozedur zur Anpassung des Dateinamens:  
Der Dateiname im *DTA-Verfahren zwischen Kunde und Bank* lautet laut Konvention der Banken DTAUS1.TXT. Nun ist es so, dass die Übertragungsprogramme auch andere Dateinamen zulassen. So kann es z.B. wünschenswert sein, den Namen des Anwenders bzw. die DTA-Laufnummer mit im Dateinamen zu haben, um die Organisation im Betrieb zu vereinfachen. Dazu dient diese Funktion.  
Es steht eine Funktion AMIC_FIBUF_DTAUS zur Verfügung, die einen neuen Dateiname generiert. Die Funktion muss zwei Parameter haben (ZahllaufId und Dateinamen), und einen Wert vom Typen Character zurückliefern. Der generierte Dateiname wir in der Tabelle AMIC_DTAUS_ASATZ im Feld DTAFileName abgespeichert. Auch bei Wiederholung der Dateiausgabe aus dem DTA-Archiv wird dieser Dateiname verwendet.

```sql
create FUNCTION AMIC_FIBUF_DTAUS( in ZahlLaufId  integer,
                                  in OldFile char(255) )
returns char(255)
begin
  declare retval char(255);
  select LEFT(OldFile,locate( OldFile, '\', -1 ) ) || 'DTAUS_' || (select Mandantname from mandantstamm) || '_'  || USER || '_'  || ZahlLaufId || '.txt'
    into retval;
  return retval;
end
```

- Ausgabedatei im Explorer anzeigen:  
Ist die Datei erstellt worden, öffnet sich der Explorer in dem Verzeichnis. Die Datei ist markiert. Dies erleichtert das Auffinden der Datei. Man kann dieses Verhalten mit diesem Einrichterparameter wieder abstellen. Dieser Einrichterparameter wird auch vom DTA-Archiv bei Wiederholung der Dateiausgabe verwendet.  
    

- IBAN mit Prüfziffernberechnung auf Gültigkeit prüfen?:  
Die IBAN wurde im Zuge der zunehmenden Internationalisierung und der Einführung des SEPA – Verfahrens nachträglich in die Datenstruktur integriert. Für die IBAN der deutschen, österreichischen und belgischen Banken existiert eine Methode zur Berechnung der Prüfziffer, die beim SEPA-Verfahren und beim Auslandszahlungsverkehr angewendet wird. Der SEPA-DTA bzw. der AuslandsDTA wird dann bei falscher IBAN abgewiesen. Diesen Test kann man hier abschalten, so dass der DTA auch mit inkorrekter IBAN durchgeführt wird.

  Dieser Test der IBAN kann auch entweder für jede [Bank](../stammdaten_zahlungsverkehr/bankenstamm.md) oder global per [Steuerparameter](../../../firmenstamm/steuerparameter/optionen_finanzwesen/iban_test_nach_standard_pruefziffernverfahren_spa_897.md) abgeschaltet werden.

- Auszuführendes VBS-Script  
Hier kann man ein Script angeben, dass nach dem Erstellen der Datei ausgeführt wird. Dieses Script wird mit dem Parameter /FILE= gefolgt von dem vollständigen Dateinamen und /ID= gefolgt von der ZAHLLAUFID aufgerufen. Dieser Einrichterparameter wird auch vom DTA-Archiv bei Wiederholung der Dateiausgabe verwendet.

- Auszuführendes VBA-Script  
Hier kann man ein Script angeben, dass nach dem Erstellen der Datei ausgeführt wird. Dieses Script wird mit dem Parameter /FILE= gefolgt von dem vollständigen Dateinamen und /ID= gefolgt von der ZAHLLAUFID aufgerufen.  
    

- Auszuführende SQL_Prozedur  
Hier kann man ein Datenbankprozedur angeben, die nach dem Erstellen der Datei ausgeführt wird. Es werden zwei Parameter übergeben: in_file= gefolgt von dem vollständigen Dateinamen und in_id= gefolgt von der ZAHLLAUFID aufgerufen.

- Auszuführender Crystal Report  
Hier kann ein Report angegeben werden. Diesem wird für den F2-Auswahlbereich die ZAHLLAUFID unter Index 1 übergeben.

- Ersteller der Zahlung darf DTA/Scheckdruck ausführen?  
Im Standardfall ist hier ein **Ja** eingetragen, so dass keine Einschränkungen vorgenommen werden. Wenn man hier ein **Nein** einträgt, kann niemals ein und dieselbe Person die Zahlung erstellen – durch Zahlvorschläge freigeben oder durch Zahlung erfassen - und ausführen. Dieser Einrichterparameter gilt gleichzeitig für Scheckdruck.  
    

- Folgende SEPA-Version verwenden  
**ACHTUNG:** *Dieser Einrichterparameter ist nicht mehr aktiv. Die Version wird jetzt pro* [Hausbank](../sepa/sepa_kennzeichen_im_hausbankenstamm.md) *gepflegt.*

- SEPA: Nur eine Art des Lastschriftverfahrens zulassen?  
Um eine Mischung von Basis- und Firmenlastschriften zu vermeiden wird vor jedem Erstellen geprüft, ob unterschiedliche Lastschriftverfahren in der Auswahl vorkommen und dann ggf. die Verarbeitung abgebrochen. Wenn man diesen Einrichterparameter auf **nein** stellt, wird diese Prüfung nicht durchgeführt.

**Hinweis zum Datenträgeraustausch Dänemark:**

Die Daten für den Datenträgeraustausch in Dänemark werden über Prozeduren (AMIC_FIBU_DK_DOMESTIC_TRANSFER für Inland und AMIC_FIBU_DK_DOMESTIC_TRANSFER für Ausland) zusammengesucht. Dies hat den Vorteil, dass Änderungen kurzfristig nachgeholt werden können, ohne extra ein Versionsupdate durchzuführen. Diese Prozeduren liefern eine Ergebnismenge zurück. Die Zeilen müssen jeweils die Feldnummer gefolgt vom Feldinhalt enthalten.

**Hinweis zum Datenträgeraustausch über eine Datenbankfunktion:**

Um der immer mehr zum Tragen kommenden Vielfalt der DTA-Verfahren in Europa entgegenzukommen, wurde eine Möglichkeit geschaffen, die Daten über eine Datenbankfunktion aufzubereiten und so auszugeben. Um diese Funktion zu nutzen, muss der SPA „DTA Ausgabeformat“ auf Datenbankfunktion stehen und in Optionen (Direktsprung **[OPT]**) muss in DTA_FUNKTION der Name der Funktion stehen. Diese Funktion erhält als Übergabeparameter die ID des Zahlungslaufes ( ASATZ_ID aus AMIC_DTAUS_ASATZ und AMIC_DTAUS_CSATZ ) und muss einen long Varchar mit dem gesamten Dateiinhalt zurückliefern.

Beispielaufbau :

```sql
create FUNCTION AMIC_DTA( in asatz_id  integer )
returns long varchar
begin
  declare retval long varchar;
  set retval = ‘001………’;
   .
   .
   .
  return retval;
end
```

<p class="just-emphasize">Inhalt des Begleitzettels im Inlandszahlungsverkehr</p>

Der einem Datenträger beizufügende Begleitzettel muss nachfolgende Mindestangaben enthalten: Dabei ist die Reihenfolge der Mindestangaben unbedingt einzuhalten; zusätzliche Angaben sind ober- oder unterhalb der geforderten Mindestangaben anzuordnen. Bei Datenträgern mit mehreren logischen Dateien ist für jede Datei ein Begleitzettel auszuschreiben.

- Begleitzettel
- Belegloser Datenträgeraustausch
- Sammelüberweisung / Sammeleinziehungsauftrag
- Vol-Nummer des Datenträgers
- Erstellungsdatum
- Anzahl der Datensätze C
- Summe Euro der Datensätze C
- Kontrollsumme der Kontonummer des Begünstigten/Zahlungspflichtigen
- Kontrollsumme der Bankleitzahlen des Begünstigten/Zahlungspflichtigen
- Bankleitzahl/Kontonummer Auftraggeber
- Name /Bankleitzahl/Kontonummer des Empfängers
- Ort, Datum
- Firma, Unterschrift(en) des Absenders

<p class="just-emphasize">Kennzeichnung des Datenträgers im Inlandszahlungsverkehr</p>

Der Datenträger ist durch Aufkleber mit folgenden Angaben zu kennzeichnen:

- Name und Bankleitzahl / Kontonummer des Absenders
- Datenträgernummer (VOL-Nummer)
- Dateiname :DTAUS1
