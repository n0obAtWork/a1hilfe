# Kachel einrichten

<!-- source: https://amic.de/hilfe/kacheleinrichten.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung [DASH] \> Variante Kachel

Auf einem bereits eingerichteten Dashboard erreicht man die Bearbeitungsmaske der Kachel auch direkt über das Kontextmenü (rechte Maustaste) des Dashboards, wenn man mit der Maus über der Kachel steht.

![Ein Bild, das Text enthält. Automatisch generierte Beschreibung](../../../ImagesExt/image8_1489.png "Ein Bild, das Text enthält. Automatisch generierte Beschreibung")

| Feldbezeichnung | Beschreibung |
| --- | --- |
| Titel | Der Titel dient als Beschreibung der Kachel. Dieser muss eindeutig sein. Zusätzlich wird eine Kachel intern über eine eindeutige Ident identifiziert. Der Titel kann jederzeit geändert werden.  
 |
| Darstellungsart | Art der Darstellung. Es existieren folgende Möglichkeiten:  
    
    
• [Text](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_text.md)  
• [Tabelle](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_tabelle.md)  
• [Fortschrittsbalken](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_fortschrittsbalken.md)  
• [Skala](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_skala.md)  
• [Säulendiagramm](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_saeulen_flaechen_und_liniendiagramm.md)  
• [Flächendiagramm](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_saeulen_flaechen_und_liniendiagramm.md)  
• [Liniendiagramm](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_saeulen_flaechen_und_liniendiagramm.md)  
• [Tortendiagramm](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_tortendiagramm.md)  
• [Bild](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_bild.md)  
• [Deutschlandkarte](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_deutschland_europakarte.md)  
• [Europakarte](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_deutschland_europakarte.md)  
• [Kombinationsdiagramm](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_kombinationsdiagramm.md)  
• [Balkendiagramm](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_balkendiagramm.md)  
• [Kalender](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_kalender.md)  
 |
| View/Prozedur | Über die private Prozedur oder View werden alle Daten geliefert, die zur Erstellung einer Kachel benötigt werden. Dabei unterscheiden sich die benötigten Daten je nach Darstellungsart. Eine genaue Beschreibung der verschiedenen Darstellungsarten findet man unter „[Prozeduren oder Views für Kacheln einrichten](./prozeduren_oder_views_fuer_kacheln_einrichten/index.md)“.  
Grundsätzlich werden Views bzw. Prozeduren ohne Parameter aufgerufen. Man der Prozedur jedoch eigene Parameter mitgeben. Dies können Konstanten, db_variablen oder LDB-Variablen (mit vorangestelltem Doppelpunkt) sein. Beispiel  
   
   
![](../../../ImagesExt/image8_1490.png)  
 |
| Refresh aktiv | Hiermit wird gekennzeichnet, ob diese Kachel auf eine „Refresh-Prozedur“ reagieren soll (Refresh aktiv = Ja) oder nicht.  
 |
|   
**Bei Klick ausführen:** Die folgenden Felder sind **nur alternativ** zu belegen. Sind mehrere Felder belegt, so wird nur die erste belegte Funktion ausgeführt. Steht der Mauszeiger auf einer Kachel mit einer Funktion, so wird das Handsymbol als Mauszeiger verwendet. |
| Funktion | Hier kann eine private oder offizielle Anwendungsfunktion (Direktsprung [ANWF]) eingetragen werden, die als Direktsprung oder aus dem Menü heraus aufgerufen werden kann. In den Funktionen kann auf die Felder **ID1** bis **ID4** über die JVars **JVAR_KACHEL_ID1** bis **JVAR_KACHEL_ID4** mit dem Owner 7659 zugegriffen werden:  
 |
| Pfleger | Aufruf eines Stammdatenpflegers, der im Pflegerstamm definiert sein muss. Für diesen Funktionsaufruf müssen die Werte, die für ID1, ID2, ID3 bzw. ID4 erwartet werden, von der View geliefert werden.  
Wenn man hier z.B. den Kundenstamm als Pfleger aufrufen möchte, so müsste in der View u.a. stehen:  
   
Ein Beispiel findet man auch unter [Geographische Karte](./prozeduren_oder_views_fuer_kacheln_einrichten/darstellungsart_deutschland_europakarte.md)  
 |
| Refresh-Prozedur | Hier kann eine Prozedur angegeben werden, mit der einzelne Kacheln des Dashboards neu angezeigt werden können. Diese Prozedur soll die idents der Kacheln zurückgeben, die neu Aufgebaut werden sollen. Dabei ist die Idee dahinter die, dass durch den Klick ein Wert in eine Datenbankvariable gesetzt wird, der dann von den anderen Kacheln ausgewertet wird. Damit kann man dann zu dem ausgewählten Datensatz noch genauere Informationen anzeigen.   
Die Datenbankprozedur hat genau sechs Parameter, die Id_board des Dashboards, die Id_kachel der auslösenden Kachel und die Identwerte, die ID1 bis ID4 entsprechen, wie sie auch vom Pfleger (siehe oben) verwendet werden. Der Typ der Ident-Parameter (interger, string, …) hängt vom Typ des ID-Feldes ab.  
Sie muss die Kachel_id der Kacheln liefern, die neu aufgebaut werden sollen.  
   
Beispielprozedur:  
In den privaten Prozeduren der Kacheln würde man dann auf pdb_adressid zugreifen:  
    
Beispielprozedur einer einfachen Textkachel (ohne jegliche Formatierung):  
 |
