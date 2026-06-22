# Sammlung der Kommandos

<!-- source: https://amic.de/hilfe/sammlungderkommandos.htm -->

Syntax

@Dateiname [PARAMETER]

Purpose

Ausführung einer Kommandodatei, Import einer XML-Datei

Anwendung

Befehlszeile, Kommandodatei

Berechtigung

Alle Anwender

Siehe auch

[Parameter](./parameter_beim_dateiaufruf_osql.md)

Beschreibung

Um Dateien von OSQL auszuführen, stellt man dem Dateinamen ein @ voran.

Ob die Datei als Kommandodatei ausgeführt oder als XML-Import ausgeführt wird, wird an der Dateinamenserweiterung festgemacht. Ist die Dateinamenserweiterung „XML“ dann wird der Import – wie unter [XMLIMPORT](./xmlimport.md) beschrieben – durchgeführt, ansonsten die Kommandodatei.

Kommandodateien werden geöffnet und es wird versucht alle dort mit Semikolon getrennten Befehle (siehe [COMMAND_DELIMITER](./set_command_delimiter_statement.md)) sequentiell (siehe [GOTO](./goto_und_label_statement.md)) abzuarbeiten. Dateien können ineinander verschachtelt werden, das heißt eine Datei kann auch eine andere Datei aufrufen. Unter OSQL können diese Dateien auch per **F3** ausgeführt werden (Abfrage Dateiname und Parameter per Dialogmaske).

Beispiel

@infile.sql

<p class="siehe-auch">Siehe auch:</p>

- [^(prototyped Funktion)](./prototyped_funktion.md)
- [ALTER STRUCT Statement](./alter_struct_statement.md)
- [ASK Statement](./ask_statement.md)
- [ASKJN Statement](./askjn_statement.md)
- [CONTINUE Statement](./continue_statement.md)
- [CONTINUE ON ERROR](./continue_on_error.md)
- [CONNECT Statement](./connect_statement.md)
- [CREATE FROM Statement](./create_from_statement.md)
- [CREATE PRIMARY KEY FROM Statement (ab Version 4.5 )](./create_primary_key_from_statement_ab_version_4_5.md)
- [CREATE STRUCT Statement](./create_struct_statement.md)
- [DBFCREATE Statement (Ab Version 5.0)](./dbfcreate_statement_ab_version_5_0.md)
- [DBFLOAD Statement](./dbfload_statement.md)
- [DISCONNECT Statement](./disconnect_statement.md)
- [DOS2WIN Statement](./dos2win_statement.md)
- [DUMP Statement](./dump_statement.md)
- [EDIT Statement](./edit_statement.md)
- [Environmentvariable](./environmentvariable.md)
- [EXIT Statement](./exit_statement.md)
- [GOTO und :LABEL Statement](./goto_und_label_statement.md)
- [IDENTLOAD Statement](./identload_statement.md)
- [IF Statement](./if_statement.md)
- [JPL Statement](./jpl_statement.md)
- [Kommentar](./kommentar.md)
- [LOAD](./load.md)
- [MAKRO oder MAKROF Statement](./makro_oder_makrof_statement.md)
- [MSG Statement](./msg_statement.md)
- [PARAMETER beim Dateiaufruf (OSQL)](./parameter_beim_dateiaufruf_osql.md)
- [PAUSE Statement](./pause_statement.md)
- [READ](./read.md)
- [SET APPEND Statement](./set_append_statement.md)
- [SET COMMAND_DELIMITER Statement](./set_command_delimiter_statement.md)
- [SET DELIMITER Statement](./set_delimiter_statement.md)
- [SET ERROR Statement](./set_error_statement.md)
- [SET KEYBOARDINTERRUPT Statement](./set_keyboardinterrupt_statement.md)
- [SET OUTERR Statement](./set_outerr_statement.md)
- [SET OUTFILE Statement](./set_outfile_statement.md)
- [SET OUTPUT Statement](./set_output_statement.md)
- [SET TITLE Statement](./set_title_statement.md)
- [SHOW BUFFER Statement](./show_buffer_statement.md)
- [SHOW CURSOR](./show_cursor.md)
- [SHOW PROCEDURE Statement](./show_procedure_statement.md)
- [SHOW TABLE Statement](./show_table_statement.md)
- [SHOW TRIGGER Statement](./show_trigger_statement.md)
- [SHOW VIEW Statement](./show_view_statement.md)
- [SHOW Statement](./show_statement.md)
- [SHOWERR Statement](./showerr_statement.md)
- [SLEEP](./sleep.md)
- [WIN2DOS](./win2dos.md)
- [XMLExport](./xmlexport.md)
- [XMLImport](./xmlimport.md)
- [ZOOM](./zoom.md)
