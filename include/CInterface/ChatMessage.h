#ifndef ChatMessage_H_
#define ChatMessage_H_

#include "OsResource.h"
#include "ZRSequence.h"
#include "TypeCode.h"
#include "InstanceHandle_t.h"
#include "CDRStream.h"
#include "ZRBuiltinTypes.h"

#ifdef __cplusplus
extern "C"
{
#endif
typedef struct ChatModule_ChatMessage
{
    DDS_Char* username; /* @ID(0) */ /* maximum length = (255) */
    DDS_Char* message; /* @ID(1) */ /* maximum length = (255) */
    DDS_Char* target_user; /* @ID(2) */ /* maximum length = (255) */
    DDS_Long timestamp; /* @ID(3) */
    DDS_Char* color; /* @ID(4) */ /* maximum length = (255) */
} ChatModule_ChatMessage; /* @Extensibility(EXTENSIBLE) */

DDS_USER_SEQUENCE_C(ChatModule_ChatMessageSeq, ChatModule_ChatMessage);

/* 用户使用接口 */
DDS_Boolean ChatModule_ChatMessageInitialize(
    ChatModule_ChatMessage* self);

DDS_Boolean ChatModule_ChatMessageInitializeEx(
    ChatModule_ChatMessage* self,
    ZRMemPool* pool,
    DDS_Boolean allocateMemory);

void ChatModule_ChatMessageFinalize(
    ChatModule_ChatMessage* self);

void ChatModule_ChatMessageFinalizeEx(
    ChatModule_ChatMessage* self,
    ZRMemPool* pool,
    DDS_Boolean deletePointers);

DDS_Boolean ChatModule_ChatMessageCopy(
    ChatModule_ChatMessage* dst,
    const ChatModule_ChatMessage* src);

DDS_Boolean ChatModule_ChatMessageCopyEx(
    ChatModule_ChatMessage* dst,
    const ChatModule_ChatMessage* src,
    ZRMemPool* pool);

void ChatModule_ChatMessagePrintData(
    const ChatModule_ChatMessage* sample);

TypeCode* ChatModule_ChatMessageGetTypeCode();

/* 底层使用函数 */
ChatModule_ChatMessage* ChatModule_ChatMessageCreateSample(
    ZRMemPool* pool,
    DDS_Boolean allocMutable);

void ChatModule_ChatMessageDestroySample(
    ZRMemPool* pool,
    ChatModule_ChatMessage* sample);

DDS_ULong ChatModule_ChatMessageGetSerializedSampleMaxSize();

DDS_ULong ChatModule_ChatMessageGetSerializedSampleSize(
    const ChatModule_ChatMessage* sample,
    DDS_ULong currentAlignment);

DDS_Long ChatModule_ChatMessageSerialize(
    const ChatModule_ChatMessage* sample,
    CDRSerializer* cdr);

DDS_Long ChatModule_ChatMessageDeserialize(
    ChatModule_ChatMessage* sample,
    CDRDeserializer* cdr,
    ZRMemPool* pool);

DDS_ULong ChatModule_ChatMessageGetSerializedKeyMaxSize();

DDS_ULong ChatModule_ChatMessageGetSerializedKeySize(
    const ChatModule_ChatMessage* sample,
    DDS_ULong currentAlignment);

DDS_Long ChatModule_ChatMessageSerializeKey(
    const ChatModule_ChatMessage* sample,
    CDRSerializer* cdr);

DDS_Long ChatModule_ChatMessageDeserializeKey(
    ChatModule_ChatMessage* sample,
    CDRDeserializer* cdr,
    ZRMemPool* pool);

DDS_Long ChatModule_ChatMessageGetKeyHash(
    const ChatModule_ChatMessage* sample,
    CDRSerializer* cdr,
    DDS_KeyHash_t* result);

DDS_Boolean ChatModule_ChatMessageHasKey();

TypeCodeHeader* ChatModule_ChatMessageGetInnerTypeCode();

#ifdef _ZRDDS_INCLUDE_ONSITE_DESERILIZE
DDS_Boolean ChatModule_ChatMessageNoSerializingSupported();

DDS_ULong ChatModule_ChatMessageFixedHeaderLength();

DDS_Long ChatModule_ChatMessageOnSiteDeserialize(CDRDeserializer* cdr,
    ChatModule_ChatMessage* sample,
    DDS_ULong offset,
    DDS_ULong totalSize,
    DDS_Char* payload,
    DDS_ULong payloadLen,
    DDS_ULong fixedHeaderLen);

#endif/*_ZRDDS_INCLUDE_ONSITE_DESERILIZE*/

#ifdef _ZRDDS_INCLUDE_NO_SERIALIZE_MODE
DDS_Char* ChatModule_ChatMessageLoanSampleBuf(ChatModule_ChatMessage* sample, DDS_Boolean takeBuffer);

void ChatModule_ChatMessageReturnSampleBuf(DDS_Char* sampleBuf);

DDS_Long ChatModule_ChatMessageLoanDeserialize(ChatModule_ChatMessage* sampleBuf,
    CDRDeserializer* cdr,
    DDS_ULong curIndex,
    DDS_ULong totalNum,
    DDS_Char* base,
    DDS_ULong offset,
    DDS_ULong space,
    DDS_ULong fixedHeaderLen);

#endif/*_ZRDDS_INCLUDE_NO_SERIALIZE_MODE*/
#ifdef __cplusplus
}
#endif
#endif
