
HEADER_FMT = '''\
#ifndef {include_guard}
#define {include_guard}

/**********************************************
*
*  @file        {filename}
*
*  @date        {date}
*
*  @author      asn1tools version {version}
*
*  @brief  	    Generated code for encoding/decoding data streams
*
* @HISTORY_START
* @HISTORY_END
*
***********************************************/

// parasoft-begin-suppress EUCHNER-CODING_RULES_7_4_4_a-3 "Generated code"
// parasoft-begin-suppress EUCHNER-CODING_RULES_7_4_5_a-3 "Generated code"

/***********************************************
* INCLUDES
***********************************************/
#include <stdbool.h>
#include "Types/eBasicTypes.h"

/***********************************************
* DATA STRUCTURES
***********************************************/
{structs}

/***********************************************
* GLOBAL FUNCTIONS DECLARATIONS
***********************************************/
{declarations}

// parasoft-end-suppress EUCHNER-CODING_RULES_7_4_4_a-3 "Generated code"
// parasoft-end-suppress EUCHNER-CODING_RULES_7_4_5_a-3 "Generated code"

#endif
'''

SOURCE_FMT = '''\
/**********************************************
*
*  @file        {filename}
*
*  @date        {date}
*
*  @author      asn1tools version {version}
*
*  @brief  	    Generated code for encoding/decoding data streams
*
* @HISTORY_START
* @HISTORY_END
*
***********************************************/

// parasoft-begin-suppress MISRA2012-RULE-8_7-4 "Generated code is not optimized to remove unreferenced functions"
// parasoft-begin-suppress MISRA2012-RULE-10_5_a-4 "Casting enums is required during deserialization"
// parasoft-begin-suppress MISRA2012-RULE-15_5-4 "Done in generated code for simplicity of functions"

// parasoft-begin-suppress EUCHNER-CODING_RULES_7_4_9_a-3 "Generated code"

/***********************************************
* INCLUDES
***********************************************/
#include <string.h>

#include "{header}"
#include "LIB/LIB_OER/eLIB_OER.h"

/***********************************************
* LOCAL FUNCTIONS DEFINITIONS
***********************************************/
{helpers}
{definitions}

// parasoft-end-suppress MISRA2012-RULE-8_7-4 "Generated code is not optimized to remove unreferenced functions"
// parasoft-end-suppress MISRA2012-RULE-10_5_a-4 "Casting enums is required during deserialization"
// parasoft-end-suppress MISRA2012-RULE-15_5-4 "Done in generated code for simplicity of functions"

// parasoft-end-suppress EUCHNER-CODING_RULES_7_4_9_a-3 "Generated code"
'''
