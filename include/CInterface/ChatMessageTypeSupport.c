/*************************************************************/
/*           此文件由编译器生成，请勿随意修改                */
/*************************************************************/
#include <stdlib.h>
#include "ZRDDSTypePlugin.h"
#include "ChatMessage.h"
#include "ChatMessageTypeSupport.h"
#include "ChatMessageDataReader.h"
#include "ChatMessageDataWriter.h"
#include "ZRDDSTypeSupport.cpp"

#ifdef __cplusplus
extern "C"
{
#endif

const DDS_Char* ChatModule_ChatMessage_TYPENAME = "ChatModule_ChatMessage";
DDSTypeSupportImpl(ChatModule_ChatMessageTypeSupport, ChatModule_ChatMessage, ChatModule_ChatMessage_TYPENAME);

DDS_TypeSupport ChatModule_ChatMessageTypeSupport_instance = {
    ChatModule_ChatMessageTypeSupport_register_type,
    ChatModule_ChatMessageTypeSupport_unregister_type,
    ChatModule_ChatMessageTypeSupport_get_type_name
};

#ifdef __cplusplus
}
#endif
