"""Building blocks for writing an Apple Shortcuts plist by hand.

Every shape here was checked against shortcuts exported from iOS, not only
against documentation; where the two disagreed, the export won.
"""

import uuid

OBJECT_REPLACEMENT = "￼"

IS = 4
CONTAINS = 99
GREATER_THAN = 2

START = 0
MIDDLE = 1
END = 2

DAYS_UNIT = 16
IS_IN_THE_LAST = 1001


def new_id():
    return str(uuid.uuid4()).upper()


def output(action_id, name):
    """The result of an earlier action, by its UUID."""
    return {"OutputUUID": action_id, "OutputName": name, "Type": "ActionOutput"}


def variable(name):
    return {"Type": "Variable", "VariableName": name}


def attachment(reference):
    """A field holding exactly one variable."""
    return {"Value": reference, "WFSerializationType": "WFTextTokenAttachment"}


def text(*parts):
    """A text field mixing literal strings and variables, in order."""
    written, attachments = "", {}
    for part in parts:
        if isinstance(part, str):
            written += part
        else:
            position = len(written.encode("utf-16-le")) // 2
            attachments[f"{{{position}, 1}}"] = part
            written += OBJECT_REPLACEMENT
    value = {"string": written}
    if attachments:
        value["attachmentsByRange"] = attachments
    return {"Value": value, "WFSerializationType": "WFTextTokenString"}


def action(identifier, **parameters):
    return {
        "WFWorkflowActionIdentifier": f"is.workflow.actions.{identifier}",
        "WFWorkflowActionParameters": parameters,
    }


class Shortcut:
    """Actions in order, plus the few control-flow shapes that need pairing."""

    def __init__(self):
        self.actions = []
        self.questions = []

    def add(self, identifier, **parameters):
        action_id = new_id()
        self.actions.append(action(identifier, UUID=action_id, **parameters))
        return action_id

    def set_variable(self, name, reference):
        self.actions.append(action("setvariable", WFVariableName=name, WFInput=attachment(reference)))

    def append_to(self, name, reference):
        self.actions.append(action("appendvariable", WFVariableName=name, WFInput=attachment(reference)))

    def ask_on_import(self, prompt, default):
        """The next Text action's contents are asked for when the shortcut is added."""
        self.questions.append(
            {
                "ActionIndex": len(self.actions),
                "Category": "Parameter",
                "DefaultValue": default,
                "ParameterKey": "WFTextActionText",
                "Text": prompt,
            }
        )

    def if_text(self, condition, subject, compared_to):
        return self._open_if(
            WFCondition=condition,
            WFConditionalActionString=compared_to,
            WFInput={"Type": "Variable", "Variable": attachment(subject)},
        )

    def if_more_than(self, subject, number):
        return self._open_if(
            WFCondition=GREATER_THAN,
            WFNumberValue=float(number),
            WFInput={"Type": "Variable", "Variable": attachment(subject)},
        )

    def _open_if(self, **condition):
        group = new_id()
        self.actions.append(
            action("conditional", GroupingIdentifier=group, WFControlFlowMode=START, **condition)
        )
        return group

    def otherwise(self, group):
        self.actions.append(action("conditional", GroupingIdentifier=group, WFControlFlowMode=MIDDLE))

    def end_if(self, group):
        self.actions.append(action("conditional", GroupingIdentifier=group, WFControlFlowMode=END))

    def for_each(self, items):
        group = new_id()
        self.actions.append(
            action("repeat.each", GroupingIdentifier=group, WFControlFlowMode=START, WFInput=attachment(items))
        )
        return group

    def end_for_each(self, group):
        self.actions.append(action("repeat.each", GroupingIdentifier=group, WFControlFlowMode=END))

    def find_health_samples(self, sample_type, days, unit=None):
        """Samples of one Health type that started within the last `days` days.

        Comes back as an error, not an empty list, when nothing matches — and an
        error stops the whole run — so only search types that always have data.
        """
        measured_in = {"WFHKSampleFilteringUnit": unit} if unit else {}
        return self.add(
            "filter.health.quantity",
            **measured_in,
            WFContentItemFilter={
                "Value": {
                    "WFActionParameterFilterPrefix": 1,
                    "WFContentPredicateBoundedDate": False,
                    "WFActionParameterFilterTemplates": [
                        {
                            "Bounded": True,
                            "Operator": IS,
                            "Property": "Type",
                            "Removable": False,
                            "Values": {
                                "Enumeration": {
                                    "Value": sample_type,
                                    "WFSerializationType": "WFStringSubstitutableState",
                                }
                            },
                        },
                        {
                            "Bounded": True,
                            "Operator": IS_IN_THE_LAST,
                            "Property": "Start Date",
                            "Removable": True,
                            "Values": {"Number": days, "Unit": DAYS_UNIT},
                        },
                    ],
                },
                "WFSerializationType": "WFContentPredicateTableTemplate",
            },
        )

    def sample_detail(self, name):
        """A detail of the sample the surrounding loop is on."""
        return self.add(
            "properties.health.quantity",
            WFContentItemPropertyName=name,
            WFInput=attachment(variable("Repeat Item")),
        )

    def adjust_date(self, date, operation, amount, unit, duration_unit):
        """`date` moved by `amount` units. Carries both the iOS 27 parameters and
        the older offset picker, since exports from both are in circulation."""
        return self.add(
            "adjustdate",
            WFDate=text(date),
            WFAdjustOperation=operation,
            WFDuration={
                "Value": {"Magnitude": str(amount), "Unit": duration_unit},
                "WFSerializationType": "WFQuantityFieldValue",
            },
            WFAdjustOffsetPicker={
                "Value": {"Operation": operation, "Unit": unit, "Value": amount},
                "WFSerializationType": "WFTimeOffsetValue",
            },
        )

    def format_day(self, date):
        return self.add(
            "format.date",
            WFDate=text(date),
            WFDateFormatStyle="Custom",
            WFDateFormat="yyyy-MM-dd",
        )

    def as_plist(self, name):
        return {
            "WFWorkflowActions": self.actions,
            "WFWorkflowClientVersion": "2700.0.4",
            "WFWorkflowMinimumClientVersion": 900,
            "WFWorkflowMinimumClientVersionString": "900",
            "WFWorkflowIcon": {
                "WFWorkflowIconGlyphNumber": 59511,
                "WFWorkflowIconStartColor": 4282601983,
            },
            "WFWorkflowName": name,
            "WFWorkflowHasOutputFallback": False,
            "WFWorkflowImportQuestions": self.questions,
            "WFWorkflowInputContentItemClasses": [],
            "WFWorkflowOutputContentItemClasses": [],
            "WFWorkflowTypes": [],
        }


def headers(*pairs):
    return {
        "Value": {
            "WFDictionaryFieldValueItems": [
                {"WFItemType": 0, "WFKey": text(key), "WFValue": value} for key, value in pairs
            ]
        },
        "WFSerializationType": "WFDictionaryFieldValue",
    }
