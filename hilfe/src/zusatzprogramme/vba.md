# Vba

<!-- source: https://amic.de/hilfe/vba.htm -->

Hauptmenü > Administration > Makroverarbeitung > Scripting

Direktsprung **[VBA]**

**In der Variante „Scripte“ werden die VBA-Scripte von A.eins gepflegt.**

**Scripte deren Namen mit „AMIC“ anfangen werden ausgeliefert.**

| Felder | |
| --- | --- |
| Name | Script-Identifikation |
| Thema | Beschreibung |
| Lokal | Wird nicht mehr unterstützt. |
| Typ | Typisierungsmöglichkeit des Scriptes. |
| geändert | Zeitstempel der letzten Änderung |
| Version | Möglichkeit der Versionierung. |
| Autor | Möglichkeit einen Autor anzugeben. |
| Größe | Größe des Scriptes in Bytes. |

| Suchen | |
| --- | --- |
| Id | Von – bis |
| Suchen … | Sucht in den Feldern<br>Name, Thema und dem VBA-Script-Text. |

| Funktionen | |
| --- | --- |
| ***Filter / bereichsauswahl***<br>***F2*** | |
| ***Duplizieren***<br>**Shift + F10** | Bietet die Möglichkeit ein Duplikat des Scriptes anzulegen. |
| ***Export*** | Exportiert ein Script |
| ***Ändern, Ansehen, Löschen, Neu*** | Standard-Pflege-Operationen |
| ***Ausführen***<br>**F9** | Führt das VBA-Script aus. |

VBA bedient folgende interne Schnittstelle:

namespace VisualBasicAeins.Scripting.Interface

{

 /// &lt;summary>

 /// Aeins-Vba-Schnittstelle

 /// &lt;/summary>

 public interface IScriptAeins

 {

 /// &lt;summary>

 /// Param

 /// &lt;/summary>

 /// &lt;param name="s">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string Param(string s);

 /// &lt;summary>

 /// Jpp_Create

 /// &lt;/summary>

 /// &lt;param name="jpp_class">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string Jpp_Create(string jpp_class);

 /// &lt;summary>

 /// Jpp_New

 /// &lt;/summary>

 /// &lt;param name="jpp_hdl">&lt;/param>

 /// &lt;param name="jpp_class">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int Jpp_New(string jpp_hdl, string jpp_class);

 /// &lt;summary>

 /// Jpp_Delete

 /// &lt;/summary>

 /// &lt;param name="jpp_hdl">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int Jpp_Delete(string jpp_hdl);

 /// &lt;summary>

 /// Jpp_Ex

 /// &lt;/summary>

 /// &lt;param name="jpp_handle">&lt;/param>

 /// &lt;param name="jpp_method">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int Jpp_Ex(string jpp_handle, string jpp_method);

 /// &lt;summary>

 /// Jpp_Do

 /// &lt;/summary>

 /// &lt;param name="jpp_handle">&lt;/param>

 /// &lt;param name="jpp_function">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string Jpp_Do(string jpp_handle, string jpp_function);

 /// &lt;summary>

 /// Jpp_In

 /// &lt;/summary>

 /// &lt;param name="jpp_handle">&lt;/param>

 /// &lt;param name="jpp_parameter">&lt;/param>

 /// &lt;param name="jpp_parameter_value">&lt;/param>

 void Jpp_In(string jpp_handle, string jpp_parameter, string jpp_parameter_value);

 /// &lt;summary>

 /// QuitApp

 /// &lt;/summary>

 /// &lt;param name="status">&lt;/param>

 void QuitApp(int status = 0);

 /// &lt;summary>

 /// Hinweis

 /// &lt;/summary>

 /// &lt;param name="text">&lt;/param>

 void Hinweis(string text);

 /// &lt;summary>

 /// Warnung

 /// &lt;/summary>

 /// &lt;param name="text">&lt;/param>

 void Warnung(string text);

 /// &lt;summary>

 /// Fehler

 /// &lt;/summary>

 /// &lt;param name="text">&lt;/param>

 void Fehler(string text);

 /// &lt;summary>

 /// Statustext

 /// &lt;/summary>

 /// &lt;param name="text">&lt;/param>

 void Statustext(string text);

 /// &lt;summary>

 /// Inputbox

 /// &lt;/summary>

 /// &lt;param name="text">&lt;/param>

 /// &lt;param name="titel">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string Inputbox(string text, string titel);

 /// &lt;summary>

 /// Sleep

 /// &lt;/summary>

 /// &lt;param name="idle">&lt;/param>

 void Sleep(int idle);

 /// &lt;summary>

 /// ModalScreen

 /// &lt;/summary>

 /// &lt;param name="screen">&lt;/param>

 /// &lt;param name="x">&lt;/param>

 /// &lt;param name="y">&lt;/param>

 /// &lt;param name="hide">&lt;/param> 

 void ModalScreen(string screen, int x, int y, bool hide);

 /// &lt;summary>

 /// GetMaskField

 /// &lt;/summary>

 /// &lt;param name="fieldName">&lt;/param>

 /// &lt;param name="occ">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string GetMaskField(string fieldName, int occ);

 /// &lt;summary>

 /// SetMaskField

 /// &lt;/summary>

 /// &lt;param name="fieldName">&lt;/param>

 /// &lt;param name="occ">&lt;/param>

 /// &lt;param name="fieldValue">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int SetMaskField(string fieldName, int occ, string fieldValue);

 /// &lt;summary>

 /// ScriptParam

 /// &lt;/summary>

 /// &lt;param name="strParam">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string ScriptParam(string strParam);

 /// &lt;summary>

 /// JVARS_IDENT

 /// &lt;/summary>

 /// &lt;returns>&lt;/returns>

 int JVARS_IDENT();

 /// &lt;summary>

 /// JVARS_BAG

 /// &lt;/summary>

 /// &lt;returns>&lt;/returns>

 int JVARS_BAG();

 /// &lt;summary>

 /// JVARS_GET

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string JVARS_GET(int jowner, string jname);

 /// &lt;summary>

 /// JVARS_SET

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;param name="jvalue">&lt;/param>

 void JVARS_SET(int jowner, string jname, string jvalue);

 /// &lt;summary>

 /// JVARS_POP

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 void JVARS_POP(int jowner, string jname);

 /// &lt;summary>

 /// JVARS_PUSH

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;param name="jvalue">&lt;/param>

 void JVARS_PUSH(int jowner, string jname, string jvalue);

 /// &lt;summary>

 /// JVARS_STACKSIZE

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int JVARS_STACKSIZE(int jowner, string jname);

 /// &lt;summary>

 /// JVARS_CLONE

 /// &lt;/summary>

 /// &lt;param name="from_owner">&lt;/param>

 /// &lt;param name="from_name">&lt;/param>

 /// &lt;param name="to_owner">&lt;/param>

 /// &lt;param name="to_name">&lt;/param>

 void JVARS_CLONE(int from_owner, string from_name, int to_owner, string to_name);

 /// &lt;summary>

 /// JVARS_EXIST

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int JVARS_EXIST(int jowner, string jname);

 /// &lt;summary>

 /// JVARS_UNSET

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 /// &lt;param name="jname">&lt;/param>

 void JVARS_UNSET(int jowner, string jname);

 /// &lt;summary>

 /// BAG_GET

 /// &lt;/summary>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string BAG_GET(string jname);

 /// &lt;summary>

 /// BAG_SET

 /// &lt;/summary>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;param name="jvalue">&lt;/param>

 void BAG_SET(string jname, string jvalue);

 /// &lt;summary>

 /// BAG_NAME

 /// &lt;/summary>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string BAG_NAME(string jname);

 /// &lt;summary>

 /// JVARS_KEYNAME

 /// &lt;/summary>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 string JVARS_KEYNAME(string jname);

 /// &lt;summary>

 /// JVARS_DEBUG

 /// &lt;/summary>

 /// &lt;param name="jowner">&lt;/param>

 void JVARS_DEBUG(int jowner);

 /// &lt;summary>

 /// BAG_UNSET

 /// &lt;/summary>

 /// &lt;param name="jname">&lt;/param>

 void BAG_UNSET(string jname);

 /// &lt;summary>

 /// BAG_EXIST

 /// &lt;/summary>

 /// &lt;param name="jname">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int BAG_EXIST(string jname);

 /// &lt;summary>

 /// vba

 /// &lt;/summary>

 /// &lt;param name="script">&lt;/param>

 /// &lt;param name="parameter">&lt;/param>

 /// &lt;returns>&lt;/returns>

 int vba(string script, string parameter = "");

 /// &lt;summary>

 /// CHECK

 /// &lt;/summary>

 /// &lt;param name="ist">&lt;/param>

 /// &lt;param name="soll">&lt;/param>

 /// &lt;param name="source">&lt;/param>

 /// &lt;param name="hinweis">&lt;/param>

 /// &lt;param name="fehlernummer">&lt;/param>

 void CHECK(string ist, string soll, string source, string hinweis, int fehlernummer);

 /// &lt;summary>

 /// NETCALL

 /// &lt;/summary>

 /// &lt;param name="the_assembly">&lt;/param>

 /// &lt;param name="the_class">&lt;/param>

 /// &lt;param name="the_method">&lt;/param>

 /// &lt;param name="the_owner">&lt;/param>

 void NETCALL(string the_assembly, string the_class, string the_method, int the_owner);

 /// &lt;summary>

 /// NETDIALOG

 /// &lt;/summary>

 /// &lt;param name="the_assembly">&lt;/param>

 /// &lt;param name="the_class">&lt;/param>

 /// &lt;param name="the_method">&lt;/param>

 /// &lt;param name="the_owner">&lt;/param>

 void NETDIALOG(string the_assembly, string the_class, string the_method, int the_owner);

 /// &lt;summary>

 /// Ausführen eines Controlstrings.

 /// &lt;/summary>

 /// &lt;param name="controlstring">&lt;/param>

 /// &lt;param name="modal">&lt;/param>

 void ControlString(string controlstring, bool modal);

 }

}
