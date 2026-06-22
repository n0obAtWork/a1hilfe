# Drucker

<!-- source: https://amic.de/hilfe/drucker.htm -->

Über **[OSQL]** können Drucker eingerichtet werden:

| Epson_bon.sql | für den Bondruckkanal eines EPSON TM930 Models. |
| --- | --- |
| Epson_schacht.sql | für den Bonschachtkanal eines EPSON TM930 Models. |
| Oki_bon_sql | für den Bondruckkanal eines OKI POS90 Bondruckers. |
| Oki_schacht.sql | für den Bonschachtkanal eines OKI POS90 Bondruckers. |
| Sni_bon.sql | für den Bondruckkanal eines SNI ND69 Bondruckers. |
| Sni_schacht.sql | für den Bonschachtkanal eines SNI ND69 Bondruckers. |
| Star.sql | für den Bondruckkanal eines Star-Druckers (dort gibt es keinen Schacht). |

Dabei ist dann nur eine freie Druckernummer einzugeben. So entspricht der bon.sql dem Drucker für die Bonrolle und der schacht.sql dem Drucker für den Schacht.

Bemerkung: Auf der Basisdatenbank existieren die Einrichtungen für EPSON und SNI-Drucker.

Wenn auch der Bonschacht für Scheckdruck,... genutzt werden soll, empfiehlt sich folgende Vorgehensweise: in der Druckerzuordnung muss der Schacht ausgewählt sein, der normale Druck auf der Bonrolle erfolgt durch entsprechende Zuordnung einer Vorgangsdruckklasse für den entsprechenden Kunden; dort kann evtl. auch festgelegt werden, dass bei Änderung der Kundennummer dann auch ein großes Formular auf dem Schacht gedruckt wird.

Um den Scheckdruck bzw. den Lastschriftdruck nutzen zu können, können Formulare über scheck.sql bzw. lastschrift.sql eingespielt werden. Für die Umwandlung des Scheckbetrages in Worten muss folgendes eingerichtet sein:

```sql
//eine private
Datenbankprozedur    P_BETRAGTEXT_SQLK:
// Private FUNKTION P_BETRAGTEXT_SQLK
CREATE FUNCTION P_BETRAGTEXT_SQLK (betrag char(20) )
RETURNS varchar(256)
//
BEGIN
//
  DECLARE ergebnis varchar(256);
  DECLARE zwtext varchar(256);
  DECLARE lg smallint;
  DECLARE l_pos smallint;
  DECLARE no_digit integer;
  DECLARE erste_ziffer integer;
  DECLARE ziffer char(6);
//
  SET ergebnis = '***';
   IF (betrag = '') THEN
     SET zwtext = '0';
   ELSE
     SET zwtext = betrag;
   END IF;
   SET lg = LENGTH(zwtext);
   SET l_pos = 1;
   SET erste_ziffer = 0;
//
  arbeit::
   LOOP
//  Eine Ziffer convertieren
    SET no_digit = 0;
    CASE SUBSTR(zwtext,l_pos,1)
     when '0' then
       SET ziffer =
'null'
     when '1' then
      SET ziffer = 'eins'
     when '2' then
      SET ziffer = 'zwei'
     when '3' then
      SET ziffer = 'drei'
     when '4' then
      SET ziffer = 'vier'
     when '5' then
      SET ziffer = 'fuenf'
     when '6' then
      SET ziffer = 'sechs'
     when '7' then
      SET ziffer =
'sieben'
     when '8' then
      SET ziffer = 'acht'
     when '9' then
      SET ziffer = 'neun'
     when ',' then
      SET ziffer =
'Komma';
     ELSE
      SET no_digit = 1;
    END CASE;
// Ende feststellen
    IF (no_digit = 0 ) THEN
       if ( erste_ziffer
= 1 ) then
         SET
ergebnis = STRING( ergebnis,'-' );
       ELSE
         SET
erste_ziffer = 1;
       END IF;
       SET ergebnis =
STRING( ergebnis,ziffer);
    END IF;
    IF (l_pos >= lg ) THEN
       LEAVE arbeit;
    ELSE
     SET l_pos = l_pos + 1;
    END IF;
   END LOOP;
   SET ergebnis = STRING( ergebnis,'***'
);
  RETURN ergebnis;
//
END
```

```sql
//ein privater
SQL-Text    sqlk_BelegBetrag:
// Privater SQL Text
sqlk_BelegBetrag     ---
select
P_BETRAGTEXT_SQLK(':BelegBetrag') Wortbetrag
```

Auf der Basisdatenbank existieren diese privaten Einstellungen

Um den Scheckdruck endgültig zu aktivieren, muss auf der Zahlungsmaske / Maske der POS-Kasse der EPA bzgl. Scheckdrucks auf „Ja“ gesetzt sein.

Es kann ein Kassenausfallsystem eingerichtet werden, um trotz Ausfall der DB an einem Server durch Nutzung eines zweiten Servers und einer gesicherten DB Kassenvorgänge durchzuführen, die mitprotokolliert werden und dann später ins restaurierte Hauptsystem wieder eingespielt werden können. (Hierzu siehe gesondertes Einrichtungsdokument, demnächst fertig)

Nach Einrichten von speziellen Bedienerklassen für die Kassenarbeitsplätze kann man durch Sperren von Funktionen für diese ausgewählten Bedienerklassen erreichen, dass nur Kassenarbeiten durchgeführt werden können.

Wenn der SPA 51 in der Gruppe Kasse/Barverkauf „Buchung der Zahlungsmittel auf Kostenkonto“ gesetzt ist, bedeutet dieses, dass beim Kassenabschluss automatisch die erfassten Zahlungsmittel auf entsprechende Kostenkonten verteilt werden. Hierzu sind entsprechende Kostenkonten in der FiBu anzulegen und die Zuordnung dieser Kostenkonten auf die Zahlungsart in der Anwendung Barvorgänge/Kasseneinstellungen, Gruppe Konten zu treffen. Hier besteht auch die Möglichkeit, die Zahlungsmittel einzeln oder komplett zu buchen (je nach EPA-Einstellung „Einzelbuchung pro Zahlungsmittel“ bei Kasseneröffnung/Abschluss)

Für Geldeinzahlung/ Geldauszahlung/ Geldentnahme/ Einreichung/ Sortenwechsel/ Zahlungsmeldung können eigene Formulare entworfen werden, die dann auf dem Schacht ausgedruckt werden. Hierfür ist auf der Zahlungsmaske der EPA ‚Sollen Einzahlungen,... über Formularsteuerung gedruckt werden‘ auf Ja zu setzen. Dann können Formulare 51-54 im Formularwesen hinterlegt werden. Die definierten Druckpositionen stehen in den folgenden Aufstellungen:
