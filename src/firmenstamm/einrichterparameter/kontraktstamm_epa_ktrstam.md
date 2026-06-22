# Kontraktstamm (EPA KTRSTAM)

<!-- source: https://amic.de/hilfe/_EPA_KTRSTAM.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Startreiter im Änderungsmodus | \-- | Hier kann die Registerkarte festgelegt werden, auf der man im Änderungsmodus steht. |
| Vorbelegung, ob Artikel Zu-/Abschläge erlaubt sind | Nein | Hier kann die Vorbelegung für das Feld „Artikel-Zu-/Ab“ festgelegt werden. |
| Vorbelegung Ausgangsrechnung oder Ausgangsgutschrift | Ausgangsrechnung | Dieser Einrichterparameter steht im Zusammenhang mit Einrichterparameter „[Umwandlung nach Fremdware/Fremdlager](./kontraktstamm_epa_ktrstam.md#epa_ktrstam_umwandlung_fremdware)“.  
Mit diesem Parameter wird festgelegt, was für ein Beleg bei der Umwandlung von einem Verkaufskontrakt erzeugt wird. |
| Vorbelegung Eingangsrechnung oder Eingangsgutschrift | Eingangsrechnung | Dieser Einrichterparameter steht im Zusammenhang mit Einrichterparameter „[Umwandlung nach Fremdware/Fremdlager](./kontraktstamm_epa_ktrstam.md#epa_ktrstam_umwandlung_fremdware)“.  
Mit diesem Parameter wird festgelegt, was für ein Beleg bei der Umwandlung von einem Einkaufskontrakt erzeugt wird. |
| Vorbelegung für die Abbuchungsmengen bei Rohwarekontrakten | \--- | Hier kann die Vorbelegung für das Feld Abbuchungsmenge angegeben werden. Bei 0 (---) wird der Standardwert „Netto“ verwendet. |
| Bausteine im Korrekturmodus automatisch auflösen? | Nein | |
| Name der Funktion für die Bezeichnung | | Hier kann eine Funktion hinterlegt werden, in der die Bezeichnung des Kontraktes geändert werden kann.  
Übergabeparameter sind  
\- Kontraktklasse  
\- Kontraktunterklasse  
\- Kundennummer  
\- Kontraktnummer  
\- Kontrakt ID |
| Sollen die Kontraktdatumfelder auf dem Reiter Konditionen angezeigt werden? | Nein | Hiermit kann festgelegt werde auf welcher Registerkarte sich die Kontraktdatumsfelder befinden. |
| Dispokennzeichen Feldstatus | aktiv | |
| Benutzer des Hedgeaccounts | TESTACC | Wird benötigt für VAX |
| Hedge Felder anzeigen | Nein | Sollen die Hedge-relevanten Felder auf der Kontraktmaske angezeigt werden. |
| Server - IP wohin der Hedge String gesendet werden soll | 127.0.0.1 | Eine IP Adresse an die die Hedge Zeichenkette gesendet werden soll |
| Port des Server wohin der Hedge String gesendet werden soll | 1045 | Port der IP Adresse |
| Maximale Durchläufe bis zum erfolgreichen Senden des Hedge Strings | 1 | Gibt an wie oft hintereinander versucht werden soll, den Hedge String an die Server IP Adresse zu senden. |
| Bemerkung Feldstatus | aktiv | |
| Drucktext 1 Feldstatus | aktiv | |
| Drucktext 2 Feldstatus | aktiv | |
| Laufzeit von/bis Feldstatus | aktiv | |
| Standardparität Feldstatus | aktiv | |
| Kontraktdruck Feldstatus | aktiv | |
| Währung Feldstatus | aktiv | |
| Zahlungsbedingung Feldstatus | aktiv | |
| Pfad zur Speicherung von Hedge-Order-Dateien | ..\\USER\\ | Pfad für die Hedgeimportschnittstelle |
| Kreditlimitüberwachung aktiv | Nein | |
| Erfassungsdatum Feldstatus | aktiv | |
| Bei automatischer Gruppenanlage ist die Kundennummer gleich die Gruppennummer | Ja | Hiermit kann festgelegt werden, ob bei der automatischen Gruppenanlage die Kundennummer und Gruppennummer gleich sein müssen und ob dies mit einer Abfrage erfolgen soll. |
| Bezeichnung für den freien Drucktext 1 | | Hier kann die Beschriftung des Feldes „Drucktext 1“ festgelegt werden. |
| Bezeichnung für den freien Drucktext 2 | | Hier kann die Beschriftung des Feldes „Drucktext 2“ festgelegt werden. |
| Bezeichnung für das Feld Matchcode | Matchcode | Hier kann die Beschriftung des Feldes „Matchcode“ festgelegt werden. |
| Vorbelegung, ob Artikel lagerspezifisch sind | Nein | Hier kann die Vorbelegung für das Feld „Lagerspezifisch“ festgelegt werden. |
| Lieferadresse Feldstatus | versteckt | |
| Matchcode Feldstatus | aktiv | |
| Soll Mengeneinheit 0 erlaubt sein | Nein | Wenn der Einrichterparameter auf „Ja“ gesetzt wird, erscheint keine Fehlermeldung bei der Mengeneinheit „0“. |
| Mengeneinheits Feldstatus | aktiv | |
| Nachkomma Feldstatus | aktiv | |
| DB-Funktion zur Erstellung des Orderstrings | AMIC_HEDGE_GETORDERSTRING | Eigene Prozedur die für das Erstellen eines Hedge Zeichenkette. Standardprozedur ist AMIC_HEDGE_GETORDERSTRING |
| Referenznummer Feldstatus | aktiv | |
| Reportzuschlag Feldstatus | aktiv | |
| Laufzeit Maximum Feldstatus | aktiv | |
| Standardlager, wenn Artikel lagerunspezifisch sind | | Hier kann das Standardlager für Artikel angegeben werden, wenn der Kontrakt als lagerunspezifisch wird dieses Lager beim Hinzufügen von Artikeln beachtet. |
| Standard Varianten Feldstatus | aktiv | Hier wird angegeben, ob das Feld Standardkontraktvariante pflegbar (aktiv), nicht pflegbar (passiv), oder unsichtbar(versteckt) ist. |
| Soll der Tabreiter Konditionen versteckt sein | Nein | Mit diesem Einrichterparameter kann die Registerkarte „Konditionen“ werden. |
| Soll der Tabreiter Konstanten versteckt sein | Nein | Mit diesem Einrichterparameter kann die Registerkarte „Konstanten“ werden. |
| Soll der Tabreiter Zusätze versteckt sein | Nein | Mit diesem Einrichterparameter kann die Registerkarte „Zusätze“ werden. |
| Soll die Zeitraumänderungsmeldung nicht angezeigt werden? | Nein | Hiermit kann die Meldung, dass die Zeiträume sich geändert haben und eventuell die Mengen angepasst werden müssen, abgeschaltet werden. |
| Überziehungszuschlag Feldstatus | aktiv | |
| Soll die Umwandlung nach Fremdware/Fremdlager erlaubt sein? | Nein | Hier kann festgelegt werden, ob die Funktionen zur Umwandlung des Kontrakts in einen „Fremdware-/Fremdlagerkontrakt“ in der Optionbox angezeigt werden. |
| Sperrkennzeichen deaktivieren | Nein | Mit dieser Option kann das Sperrkennzeichen von der Maske entfernt werden, um Fehleingaben zu vermeiden. |
| Soll die Kontraktreorganisationsmeldung deaktiviert sein? | Nein | Hiermit kann die Meldung, dass eine Kontraktreorganisation durchgeführt werden muss, ausgeschaltet werden. |
