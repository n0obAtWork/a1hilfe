# Bearbeiten Sammeldruck

<!-- source: https://amic.de/hilfe/_rwsammelbearbeiten.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung

Direktsprung **[RWBV]**

Die Auswahlvariante ‚*Bearbeiten Sammeldruck*‘ im Einkauf und im Verkauf stellt Funktionen zur Verfügung, um bestehende Rohware-Sammeldruck-Belege erneut zu drucken, Storno-Sammeldruck-Belege zu erstellen und zu drucken, Sammeldruckbelege wieder aufzuheben, eine Auswahlliste der Einzelbelege eines Sammeldruckbelegs aufzurufen und Ergänzungswerte aller Einzelbelege eines Sammeldruckbeleges zu bearbeiten.

Für die Zusammenstellung der Auswahlliste ist eine möglichst gezielte Angabe der Kriterien der Bereichsauswahl vorzunehmen, da die Informationsbeschaffung für die Darstellung der virtuellen Sammelbelege aus den zugehörigen Einzelbelegen unter Umständen längere Antwortzeiten des Datenbanksystems bedeuten kann.

**Druck**

Die Funktion **Druck** dient dem wiederholten, bei Stornobelegen auch dem ersten, Ausdruck der ausgewählten Sammeldruckbelege.

**Einzelbelege zeigen**

Ist genau ein Sammeldruckbeleg markiert (ausgewählt), so kann mit **Einzelbelege zeigen** eine Auswahlliste mit den zugehörigen Einzelbelegen aufgerufen werden, die wiederum die Ansicht und Vorschau dieser Belege ermöglicht.

**Druck zurücksetzen**

Die Funktion **Druck zurücksetzen** löst einen Sammeldruckbeleg wieder auf. Dieses kann nur für Belege erfolgen, die noch nicht in die Finanzbuchhaltung übertragen wurden. Sie ist dann auszuführen, wenn an Einzelbelegen eines Sammeldrucks noch Änderungen vorzunehmen sind oder die Sammeldruckzusammenstellung nicht das gewünschte Ergebnis erbracht hat. Je nach Einstellung des Rohwareparameters [RWPA] [*Sammelnummer-Release bei Druckrücksetzen*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_163) wird die Drucknummer (Sammelbelegnummer) dabei in den Nummernkreis zurückgeschrieben oder nicht.

**Rohware-Mail erneut versenden**

Sollen beim Erstellen von Rohwaresammelbelegen erstellte E-Mails bezüglich der E-Mail-Empfänger geändert werden, können die relevanten Belege markiert werden und die Funktion **Rohware-Mail erneut versenden** aufgerufen werden. Dadurch öffnet sich ein Pfleger, mit dem man die E-Mails entsprechend pflegen und neu versenden kann.

**Sammel-Storno erstellen**

Um Stornobelege zu Sammeldruckbelegen zu erzeugen, wird die Funktion **Sammel-Storno erstellen** aufgerufen. Hier wird zu jedem ausgewählten Sammeldruckbeleg ein Sammeldruckstornobeleg, und damit zu jedem enthaltenen Einzelbeleg ein Einzelstornobeleg, erzeugt. Auch für diese Funktion gibt es eine Reihe von Rohwareparametern [RWPA] und Einstellungen auf der Steuerungsmaske, die das Verhalten steuern und Einfluss auf bestimmte Werte der erzeugten Stornobelege haben.  
Zunächst bestimmt der Parameter [*Sammel-Storno nur nach Fibuübertrag*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_145) ob die Stornobeleg-Erzeugung auch zu nicht gebuchten Belegen erfolgen darf.  
Der Parameter [*Sammel-Storno-Beleg-/Druck-Datum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_140) legt fest, ob die Stornobelege mit dem Druck-/Sammelbelegdatum der Quell-Sammeldruckbelege erstellt werden sollen oder dieses auf der Steuerungsmaske von Fall zu Fall einzustellen ist.  
Dabei ist zu beachten, dass in der Variante Stornobelegerstellung ohne Kopie des Originalbelegs das ggf. angegebene Belegdatum ebenso wie die ggf. angegebene Periode im selben Geschäftsjahr wie das des zu stornierenden Belegs liegen muss.  
Mit dem Parameter [*Perioden bei Sammel-Stornobelegen*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_144) wird die Periodenbehandlung der Stornobelege festgelegt. Es können die Originalperioden erhalten bleiben oder die Einstellung auf der Maske vorgenommen werden. Es ist jedoch zu beachten, dass bei unpassenden Angaben von Periode und Jahr automatische Perioden-/Jahr-Anpassungen mit entsprechendem Hinweis vom System vorgenommen werden.  
Der Parameter [*Sammel-Storno-Belegnummer*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_141) regelt die Vergabe der Sammelstorno-Druck-/Belegnummer. Ist die Einstellung hier ‚*Originalnummer*‘, so wird die Drucknummer des zu stornierenden Beleges auch für den Stornobeleg herangenutzt. Dabei ist jedoch darauf zu achten, dass die laut in den Mandantennummernkreis zugeordneten Zählkreise für die Stornobelege auch den Nummernbereich enthalten, der die möglichen Originalnummern enthält.  
Unabhängig vom Druck-/Sammelbelegdatum des Stornobelegs kann mit dem Parameter [*Sammel-Storno-Einzelbeleg-Datum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_142) festgelegt werden, ob das Belegdatum der Einzelstornobelege eines Sammelstornobelegs erhalten bleiben soll oder das Druck-/Sammelbelegdatum erhalten sollen.  
Die Druck-Formularnummer des Sammelstornobelegs berechnet sich aus der Druckformularnummer des Original-Sammeldruckbeleges zuzüglich einem Offset, der im Rohwareparameter [RWPA] [*Sammelstornoformular-Offset*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_146) festgelegt ist. Unter der so bestimmten Nummer sollte dann auch ein Sammeldruckformular eingerichtet sein.

 Das Feld Stornodatumverhalten ist nur bei dem Wert Ja im Feld Storno nach Kopie änderbar. Standardmäßig steht dort der Wert kein manuelles Valutadatum.

Wenn dieser Wert auf Stornobeleg mit manuellem Valutadatum oder Stornobeleg und Kopie mit manuellem Valutadatum geändert wird, dann wird das manuelle Valutadatum Feld mit dem heutigen Datum vorbelegt und wird bearbeitbar. Falls vorhanden, wird eine in das Feld manuelle Zahlungsbedingung eingerichtete Zahlungsbedingung gezogen mit dem Typ Datum manuell eingebbar und das Feld wird bearbeitbar.  
Dadurch werden je nach Wert im Feld Stornodatumverhalten nur der Stornobeleg oder auch die Belegkopie mit dem manuellem Valutadatum erstellt.

Zusätzlich zur Erstellung von Sammelstornobelegen kann der ursprüngliche Sammeldruckbeleg unter dem angegeben Datum kopiert werden. Diese Option ist aber nur freigeschaltet, wenn der Steuerungsparameter [SPA] [*Rohware-Storno mit Quellbeleg-Kopie*](../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/rohwarestorno_mit_quellbeleg_kopie_spa_654.md) mit dem Wert ‚*erlaubt‘* eingestellt ist. Vorbelegt mit dem Wert des Rohwareparameters [*Stornobelege: Vorbelegung: >mit Kopie&lt;*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_171) kann dieses dann auf der Steuerungsmaske festgelegt werden.  
Eine weitere Option, freigeschaltet durch den Steuerungsparameter [SPA] [*Rohwarestorno mit Alternativen bez. Fibustatus*](../../firmenstamm/steuerparameter/vorgangsbearbeitung_umwandlung/rohwarestorno_mit_alternat_bez_fibstatus_spa_655.md) mit dem Wert ‚*erlaubt‘,* ermöglicht unterschiedliche automatische Aktionen hinsichtlich der erfolgten oder nicht erfolgten Übertragung und Buchung in die Finanzbuchhaltung. Das durch den Steuerparameter freigeschaltete Maskenfeld ‚*Stornobeleg erzeugen*‘ erlaubt die Alternativen

‚*immer*‘  
Stornobelegerzeugung erfolgt unabhängig vom Fibu-Status

‚*Nur wenn FIBU Übertrag*‘  
Es wird nur dann ein Stornobeleg erzeugt, wenn der Originalbeleg an die Finanzbuchhaltung übergeben wurde. Andernfalls wird der Originalbeleg per Sammeldruck-Rücksetzung aufgelöst. Die Original-Einzelbelege stehen damit zur Korrektur etc. zur Verfügung.

‚*Nur wenn FIBU gebucht*‘  
Es wird nur dann ein Stornobeleg erzeugt, wenn der Originalbeleg an die Finanzbuchhaltung übergeben und gebucht wurde. Andernfalls wird ggf. der Primanota-Eintrag gelöscht und der Originalbeleg per Sammeldruck-Rücksetzung aufgelöst. Die Original-Einzelbelege stehen damit zur Korrektur etc. zur Verfügung.

Bezüglich des Einflusses von Bewegungen der Stornobelege und Belegkopien auf Massebilanzen ist die Einstellung des bei der Option **Storno mit Kopie** herangezogenen Rohwareparameters [*Massebilanz bei Storno mit Kopie*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_191) zu berücksichtigen.

**Ergänzungswerte**

Mit dieser Funktion können nachträglich noch die Rohwarenergänzungsparameter bei Rohwaresammelbelege der dazugehörigen Rohwarengruppe geändert werden.

Wenn noch keine Rohwarenergänzungsparameter eingetragen worden sind, oder die Rohwarenergänzungsparameter in den einzelnen Belegen den gleichen Inhalt haben, so kann auf der Maske ein neuer Wert eingetragen werden.

Sind in den Rohwarenergänzungsparameter der Einzelbelege unterschiedliche Werte eingetragen worden, so kann erst der Wert überschrieben werden, wenn das Ankreuzfeld rechts neben dem Textfeld aktiviert wird.

Man muss dabei beachten, dass die Rohwarenergänzungsparameter für alle Einzelbelege des Sammelbeleges überschrieben werden.
