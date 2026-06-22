# Erstellen eines Hinweises

<!-- source: https://amic.de/hilfe/_hinweiserstellen.htm -->

Hauptmenü > Administration > Firmenkonstanten > Bedienerbezogenes Hinweissystem

Direktsprung **[HINW]**

<p class="just-emphasize">Allgemeine Felder</p>

| Maskenfelder | Bedeutung |
| --- | --- |
| Typ | Hier kann zwischen Archiv, Hilfe, Internet/Intranet unterschieden werden. |
| Ident | Ist der Ident für die gültige Datei. |
| Gültig ab | Hier kann das Datum angegeben werden, ab wann die Meldung zu sehen ist. Standard ist das Tagesdatum. |
| Start Zeit | Hier kann die Uhrzeit angegeben werden, ab wann der Hinweis zu sehen sein soll. Standard ist 0 Uhr. |
| Gültig bis | Hier kann das Datum eingestellt werden wie lang die Information auf der Anzeigemaske beim A.eins-Start angezeigt werden soll. Standard ist das Tagesdatum. |
| End Zeit | Hier kann die Uhrzeit eingetragen werden, bis wann die Information gültig ist. Standard ist 23:59. |

<p class="just-emphasize">Register Allgemein</p>

| Maskenfelder | Bedeutung |
| --- | --- |
| Bezeichnung | Hier kann ein Kurztext eingetragen werden, der auf der Information Anzeigemaske angezeigt werden soll. |
| Bedienerklasse | Bestimmt für welche Bedienerklasse die Information gelten soll. |
| Priorität | Hier kann zwischen „Normal“ und „Hoch“ gewählt werden. Informationen, die eine hohe Priorität haben, stehen oben auf der Anzeigemaske. |
| Bedingt durch Prozedur | Gibt eine Datenbank-Prozedur an, die entscheidet, ob der Hinweis angezeigt werden soll.  
Die Prozedur kann einen oder zwei Rückgabewerte haben. Der Rückgabewert mit Namen „**result**“ ist vom Typ Integer und muss in jedem Fall zurückgeliefert werden. Ist der Wert größer 0, so wird der Hinweis angezeigt.  
Der zweite optionale Rückgabewert mit dem Name „**result_text**“ ist vom Typ char(255) und liefert einen Variablen Text zurück, der den unter **Bezeichnung** angegebenen Text überschreibt. |

<p class="just-emphasize">Typ</p>

In Abhängigkeit des Typs muss im Feld Ident der richtige Inhalt eingetragen werden. Der Inhalt des Feldes Ident enthält die Identifikation des Dokumentes.

Wird als Typ Archiv ausgewählt, so wird in das Feld die Archiv Referenz eingetragen.

Wird als Typ Hilfe ausgewählt, so wird in das Feld der logische Name des Hilfedokuments eingetragen.

Wird als Typ Internet/Intranet ausgewählt, so wird in das Feld Ident der Link auf das HTML-Dokument eingetragen. In diesem Fall muss beachtet werden, dass der Link mit http:// beginnt. Der Link kann auf ein Dokument im Internet oder im Firmennetzwerk zeigen.

Soll ein UNC-Pfad hinterlegt werden wie z.B. **\\\\rechner\\dokumente\\test.htm**, so muss dieser Link Windows spezifisch hinterlegt werden **file://///minas/temp/bla.htm**.

Beispiel Prozedur mit einem Rückgabewert:

```sql
create procedure
HINWEIS_SPRACHTEXT
( )
result ( result integer )
begin
  declare erg integer;

  set erg =  ( select count(*) from ergebis ) ;

  select erg;
end
```

Beispiel Prozedur mit zwei Rückgabewerten:

```sql
create procedure HINWEIS_SPRACHTEXT(
)
result ( result integer,
         result_text char(255) )
begin
  declare erg integer;
  declare dc_bezeich char(255);
  declare dc_eintrag char(255);

  set erg
= 0;
-- kein Hinweitext

  for loop_presentation as
presentation_curs insensitive cursor
for
  select eintrag
     from arbeitsschritte
where bedienerid=db_bedienerid
  do
    set
erg = erg + 1;

set dc_eintrag = eintrag;
  end for ;
  if ( erg > 1) then
      set dc_bezeich = AMIC_FUNC_SPRACHTEXTE('OD', 'OD', 'Es sind noch %s
Arbeitsschritte offen.', -1, erg);
    else
      set dc_bezeich = AMIC_FUNC_SPRACHTEXTE('OD', 'OD', 'Der Arbeitsschritt %s
muss noch abgearbeitet werden.', -1, dc_eintrag );

    endif;

  select erg, dc_bezeich;

end
```

<p class="just-emphasize">Register Gelesen</p>

Auf der Registerkarte **Gelesen** werden alle Benutzer angezeigt die die Information als gelesen markiert haben. Um das Gelesen-Kennzeichen für einen Benutzer zu löschen kann hier die entsprechende Zeile markiert werden und mit der Funktion ***Lösche gelesen*** entfernt werden. Mit der Funktion ***Lösche alle gelesen*** können alle Benutzer entfernt werden, die diesen Hinweis als schon gelesen markiert haben.
