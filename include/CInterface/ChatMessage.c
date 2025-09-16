/*************************************************************/
/*           此文件由编译器生成，请勿随意修改                */
/*************************************************************/
#include "ZRMemPool.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ChatMessage.h"


#define T ChatModule_ChatMessage
#define TSeq ChatModule_ChatMessageSeq
#define TINITIALIZE ChatModule_ChatMessageInitializeEx
#define TFINALIZE ChatModule_ChatMessageFinalizeEx
#define TCOPY ChatModule_ChatMessageCopyEx

#include "ZRSequence.cpp"

#undef TCOPY
#undef TFINALIZE
#undef TINITIALIZE
#undef TSeq
#undef T

DDS_Boolean ChatModule_ChatMessageInitializeEx(
    ChatModule_ChatMessage* self,
    ZRMemPool* pool,
    DDS_Boolean allocateMemory)
{
    self->username = NULL;

    self->message = NULL;

    self->target_user = NULL;

    self->timestamp = 0;

    self->color = NULL;

    if (allocateMemory)
    {
        self->username = (DDS_Char*) ZRMalloc(pool, 255 + 1);
        if (self->username == NULL)
        {
            printf("Malloc for self->username failed.");
            return false;
        }
        self->username[0] = '\0';
        self->message = (DDS_Char*) ZRMalloc(pool, 255 + 1);
        if (self->message == NULL)
        {
            printf("Malloc for self->message failed.");
            return false;
        }
        self->message[0] = '\0';
        self->target_user = (DDS_Char*) ZRMalloc(pool, 255 + 1);
        if (self->target_user == NULL)
        {
            printf("Malloc for self->target_user failed.");
            return false;
        }
        self->target_user[0] = '\0';
        self->color = (DDS_Char*) ZRMalloc(pool, 255 + 1);
        if (self->color == NULL)
        {
            printf("Malloc for self->color failed.");
            return false;
        }
        self->color[0] = '\0';
    }
    else
    {
        if (self->username != NULL)
        {
            self->username[0] = '\0';
        }
        if (self->message != NULL)
        {
            self->message[0] = '\0';
        }
        if (self->target_user != NULL)
        {
            self->target_user[0] = '\0';
        }
        if (self->color != NULL)
        {
            self->color[0] = '\0';
        }
    }
    return true;
}

void ChatModule_ChatMessageFinalizeEx(
    ChatModule_ChatMessage* self,
    ZRMemPool* pool,
    DDS_Boolean deletePointers)
{
    if (deletePointers)
    {
        ZRDealloc(pool, self->username);
        self->username = NULL;
        ZRDealloc(pool, self->message);
        self->message = NULL;
        ZRDealloc(pool, self->target_user);
        self->target_user = NULL;
        ZRDealloc(pool, self->color);
        self->color = NULL;
    }
}

DDS_Boolean ChatModule_ChatMessageCopyEx(
    ChatModule_ChatMessage* dst,
    const ChatModule_ChatMessage* src,
    ZRMemPool* pool)
{
    if (src->username == NULL)
    {
        ZRDealloc(pool, dst->username);
        dst->username = NULL;
    }
    else
    {
        if (dst->username == NULL)
        {
            dst->username = (DDS_Char*) ZRMalloc(pool, 255 + 1);
            if (dst->username == NULL)
            {
                printf("malloc for username failed.");
                return false;
            }
        }
        strcpy(dst->username, src->username);
    }

    if (src->message == NULL)
    {
        ZRDealloc(pool, dst->message);
        dst->message = NULL;
    }
    else
    {
        if (dst->message == NULL)
        {
            dst->message = (DDS_Char*) ZRMalloc(pool, 255 + 1);
            if (dst->message == NULL)
            {
                printf("malloc for message failed.");
                return false;
            }
        }
        strcpy(dst->message, src->message);
    }

    if (src->target_user == NULL)
    {
        ZRDealloc(pool, dst->target_user);
        dst->target_user = NULL;
    }
    else
    {
        if (dst->target_user == NULL)
        {
            dst->target_user = (DDS_Char*) ZRMalloc(pool, 255 + 1);
            if (dst->target_user == NULL)
            {
                printf("malloc for target_user failed.");
                return false;
            }
        }
        strcpy(dst->target_user, src->target_user);
    }

    dst->timestamp = src->timestamp;

    if (src->color == NULL)
    {
        ZRDealloc(pool, dst->color);
        dst->color = NULL;
    }
    else
    {
        if (dst->color == NULL)
        {
            dst->color = (DDS_Char*) ZRMalloc(pool, 255 + 1);
            if (dst->color == NULL)
            {
                printf("malloc for color failed.");
                return false;
            }
        }
        strcpy(dst->color, src->color);
    }

    return true;
}

void ChatModule_ChatMessagePrintData(const ChatModule_ChatMessage *sample)
{
    if (sample == NULL)
    {
        printf("NULL\n");
        return;
    }
    if (sample->username != NULL)
    {
        printf("sample->username(%d): %s\n", strlen(sample->username), sample->username);
    }
    else
    {
        printf("sample->username(0): NULL\n");
    }
    printf("\n");

    if (sample->message != NULL)
    {
        printf("sample->message(%d): %s\n", strlen(sample->message), sample->message);
    }
    else
    {
        printf("sample->message(0): NULL\n");
    }
    printf("\n");

    if (sample->target_user != NULL)
    {
        printf("sample->target_user(%d): %s\n", strlen(sample->target_user), sample->target_user);
    }
    else
    {
        printf("sample->target_user(0): NULL\n");
    }
    printf("\n");

    printf("sample->timestamp: %d\n", sample->timestamp);
    printf("\n");

    if (sample->color != NULL)
    {
        printf("sample->color(%d): %s\n", strlen(sample->color), sample->color);
    }
    else
    {
        printf("sample->color(0): NULL\n");
    }
    printf("\n");

}

TypeCode* ChatModule_ChatMessageGetTypeCode()
{
    DDS_Long ret = 0;
    TypeCode* memberTc = NULL;
    TypeCode* eleTc = NULL;
    static TypeCode* s_typeCode = NULL;
    if (s_typeCode != NULL) return s_typeCode;

    s_typeCode = TypeCodeFactoryCreateStructTC(TypeCodeFactoryGetInstance(),
        "ChatModule_ChatMessage",
        DDS_EXTENSIBLE_EXTENSIBILITY);
    if (s_typeCode == NULL)
    {
        printf("create struct ChatModule_ChatMessage typecode failed.");
        return s_typeCode;
    }
    memberTc = TypeCodeFactoryCreateStringTC(TypeCodeFactoryGetInstance(),255);
    if (memberTc == NULL)
    {
        printf("Get Member username TypeCode failed.");
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }
    ret = TypeCodeAddMemberToStruct(
        s_typeCode,
        0,
        0,
        "username",
        memberTc,
        false,
        false);
    TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), memberTc);
    if (ret < 0)
    {
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }

    memberTc = TypeCodeFactoryCreateStringTC(TypeCodeFactoryGetInstance(),255);
    if (memberTc == NULL)
    {
        printf("Get Member message TypeCode failed.");
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }
    ret = TypeCodeAddMemberToStruct(
        s_typeCode,
        1,
        1,
        "message",
        memberTc,
        false,
        false);
    TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), memberTc);
    if (ret < 0)
    {
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }

    memberTc = TypeCodeFactoryCreateStringTC(TypeCodeFactoryGetInstance(),255);
    if (memberTc == NULL)
    {
        printf("Get Member target_user TypeCode failed.");
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }
    ret = TypeCodeAddMemberToStruct(
        s_typeCode,
        2,
        2,
        "target_user",
        memberTc,
        false,
        false);
    TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), memberTc);
    if (ret < 0)
    {
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }

    memberTc = TypeCodeFactoryGetPrimitiveTC(TypeCodeFactoryGetInstance(), DDS_TK_INT);
    if (memberTc == NULL)
    {
        printf("Get Member timestamp TypeCode failed.");
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }
    ret = TypeCodeAddMemberToStruct(
        s_typeCode,
        3,
        3,
        "timestamp",
        memberTc,
        false,
        false);
    if (ret < 0)
    {
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }

    memberTc = TypeCodeFactoryCreateStringTC(TypeCodeFactoryGetInstance(),255);
    if (memberTc == NULL)
    {
        printf("Get Member color TypeCode failed.");
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }
    ret = TypeCodeAddMemberToStruct(
        s_typeCode,
        4,
        4,
        "color",
        memberTc,
        false,
        false);
    TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), memberTc);
    if (ret < 0)
    {
        TypeCodeFactoryDeleteTC(TypeCodeFactoryGetInstance(), s_typeCode);
        s_typeCode = NULL;
        return NULL;
    }

    return s_typeCode;
}

DDS_Long ChatModule_ChatMessageSerialize(const ChatModule_ChatMessage* sample, CDRSerializer *cdr)
{
    if (!CDRSerializerPutString(cdr, (DDS_Char*) sample->username, sample->username == NULL ? 0 : strlen(sample->username) + 1))
    {
        printf("serialize sample->username failed.");
        return -2;
    }

    if (!CDRSerializerPutString(cdr, (DDS_Char*) sample->message, sample->message == NULL ? 0 : strlen(sample->message) + 1))
    {
        printf("serialize sample->message failed.");
        return -2;
    }

    if (!CDRSerializerPutString(cdr, (DDS_Char*) sample->target_user, sample->target_user == NULL ? 0 : strlen(sample->target_user) + 1))
    {
        printf("serialize sample->target_user failed.");
        return -2;
    }

    if (!CDRSerializerPutUntype(cdr, (DDS_Octet*) &sample->timestamp, 4))
    {
        printf("serialize sample->timestamp failed.");
        return -2;
    }

    if (!CDRSerializerPutString(cdr, (DDS_Char*) sample->color, sample->color == NULL ? 0 : strlen(sample->color) + 1))
    {
        printf("serialize sample->color failed.");
        return -2;
    }

    return 0;
}

DDS_Long ChatModule_ChatMessageDeserialize(
    ChatModule_ChatMessage* sample,
    CDRDeserializer* cdr,
    ZRMemPool* pool)
{
    DDS_ULong usernameTmpLen;
    DDS_ULong messageTmpLen;
    DDS_ULong target_userTmpLen;
    DDS_ULong colorTmpLen;
    /*no key*/
    usernameTmpLen = 0;
    if (!CDRDeserializerGetUntype(cdr, (DDS_Octet*) &usernameTmpLen, 4))
    {
        sample->username = NULL;
        sample->message = NULL;
        sample->target_user = NULL;
        sample->timestamp = 0;
        sample->color = NULL;
        return 0;
    }
    if (0 == usernameTmpLen)
    {
        ZRDealloc(pool, sample->username);
        sample->username = NULL;
    }
    else
    {
        if ( sample->username == NULL)
        {
            sample->username = (DDS_Char*) ZRMalloc(pool, usernameTmpLen);
            if (sample->username == NULL)
            {
                printf("malloc for sample->username failed(%d).", usernameTmpLen);
                return -3;
            }
        }
        if (!CDRDeserializerGetUntypeArray(cdr, (DDS_Octet*)sample->username, usernameTmpLen, 1))
        {
            printf("deserialize member sample->username failed.");
            return -4;
        }
    }
    messageTmpLen = 0;
    if (!CDRDeserializerGetUntype(cdr, (DDS_Octet*) &messageTmpLen, 4))
    {
        sample->message = NULL;
        sample->target_user = NULL;
        sample->timestamp = 0;
        sample->color = NULL;
        return 0;
    }
    if (0 == messageTmpLen)
    {
        ZRDealloc(pool, sample->message);
        sample->message = NULL;
    }
    else
    {
        if ( sample->message == NULL)
        {
            sample->message = (DDS_Char*) ZRMalloc(pool, messageTmpLen);
            if (sample->message == NULL)
            {
                printf("malloc for sample->message failed(%d).", messageTmpLen);
                return -3;
            }
        }
        if (!CDRDeserializerGetUntypeArray(cdr, (DDS_Octet*)sample->message, messageTmpLen, 1))
        {
            printf("deserialize member sample->message failed.");
            return -4;
        }
    }
    target_userTmpLen = 0;
    if (!CDRDeserializerGetUntype(cdr, (DDS_Octet*) &target_userTmpLen, 4))
    {
        sample->target_user = NULL;
        sample->timestamp = 0;
        sample->color = NULL;
        return 0;
    }
    if (0 == target_userTmpLen)
    {
        ZRDealloc(pool, sample->target_user);
        sample->target_user = NULL;
    }
    else
    {
        if ( sample->target_user == NULL)
        {
            sample->target_user = (DDS_Char*) ZRMalloc(pool, target_userTmpLen);
            if (sample->target_user == NULL)
            {
                printf("malloc for sample->target_user failed(%d).", target_userTmpLen);
                return -3;
            }
        }
        if (!CDRDeserializerGetUntypeArray(cdr, (DDS_Octet*)sample->target_user, target_userTmpLen, 1))
        {
            printf("deserialize member sample->target_user failed.");
            return -4;
        }
    }
    if (!CDRDeserializerGetUntype(cdr, (DDS_Octet*) &sample->timestamp, 4))
    {
        sample->timestamp = 0;
        sample->color = NULL;
        return 0;
    }
    colorTmpLen = 0;
    if (!CDRDeserializerGetUntype(cdr, (DDS_Octet*) &colorTmpLen, 4))
    {
        sample->color = NULL;
        return 0;
    }
    if (0 == colorTmpLen)
    {
        ZRDealloc(pool, sample->color);
        sample->color = NULL;
    }
    else
    {
        if ( sample->color == NULL)
        {
            sample->color = (DDS_Char*) ZRMalloc(pool, colorTmpLen);
            if (sample->color == NULL)
            {
                printf("malloc for sample->color failed(%d).", colorTmpLen);
                return -3;
            }
        }
        if (!CDRDeserializerGetUntypeArray(cdr, (DDS_Octet*)sample->color, colorTmpLen, 1))
        {
            printf("deserialize member sample->color failed.");
            return -4;
        }
    }
    return 0;
}

DDS_ULong ChatModule_ChatMessageGetSerializedSampleSize(const ChatModule_ChatMessage* sample, DDS_ULong currentAlignment)
{

    DDS_ULong initialAlignment = currentAlignment;
    currentAlignment += CDRSerializerGetStringSize(sample->username == NULL ? 0 : strlen(sample->username) + 1, currentAlignment);

    currentAlignment += CDRSerializerGetStringSize(sample->message == NULL ? 0 : strlen(sample->message) + 1, currentAlignment);

    currentAlignment += CDRSerializerGetStringSize(sample->target_user == NULL ? 0 : strlen(sample->target_user) + 1, currentAlignment);

    currentAlignment += CDRSerializerGetUntypeSize(4, currentAlignment);

    currentAlignment += CDRSerializerGetStringSize(sample->color == NULL ? 0 : strlen(sample->color) + 1, currentAlignment);

    return currentAlignment - initialAlignment;
}

DDS_Long ChatModule_ChatMessageSerializeKey(const ChatModule_ChatMessage* sample, CDRSerializer *cdr)
{
    if (ChatModule_ChatMessageSerialize(sample, cdr) < 0)
    {
        return -1;
    }
    return 0;
}

DDS_Long ChatModule_ChatMessageDeserializeKey(
    ChatModule_ChatMessage* sample,
    CDRDeserializer* cdr,
    ZRMemPool* pool)
{
    if (ChatModule_ChatMessageDeserialize(sample, cdr, pool) < 0)
    {
        return -1;
    }
    return 0;
}

DDS_ULong ChatModule_ChatMessageGetSerializedKeySize(const ChatModule_ChatMessage* sample, DDS_ULong currentAlignment)
{
    DDS_ULong initialAlignment = currentAlignment;

    currentAlignment += ChatModule_ChatMessageGetSerializedSampleSize(sample, currentAlignment);
    return currentAlignment - initialAlignment;
}

#ifdef _ZRDDS_INCLUDE_NO_SERIALIZE_MODE
DDS_Char* ChatModule_ChatMessageLoanSampleBuf(ChatModule_ChatMessage* sample, DDS_Boolean takeBuffer)
{
    return NULL;
}

void ChatModule_ChatMessageReturnSampleBuf(DDS_Char* sampleBuf)
{
    ;
}

DDS_Long ChatModule_ChatMessageLoanDeserialize(ChatModule_ChatMessage* sampleBuf,
    CDRDeserializer* cdr,
    DDS_ULong curIndex,
    DDS_ULong totalNum,
    DDS_Char* base,
    DDS_ULong offset,
    DDS_ULong space,
    DDS_ULong fixedHeaderLen)
{
    return 0;
}

#endif/*_ZRDDS_INCLUDE_NO_SERIALIZE_MODE*/

#ifdef _ZRDDS_INCLUDE_ONSITE_DESERILIZE
DDS_Long ChatModule_ChatMessageOnSiteDeserialize(CDRDeserializer* cdr,
    ChatModule_ChatMessage* sample,
    DDS_ULong offset,
    DDS_ULong totalSize,
    DDS_Char* payload,
    DDS_ULong payloadLen,
    DDS_ULong fixedHeaderLen)
{
    return 0;
}

DDS_Boolean ChatModule_ChatMessageNoSerializingSupported()
{
    return false;
}

DDS_ULong ChatModule_ChatMessageFixedHeaderLength()
{
    return 0;
}

#endif/*_ZRDDS_INCLUDE_ONSITE_DESERILIZE*/
DDS_Boolean ChatModule_ChatMessageInitialize(ChatModule_ChatMessage* self)
{
    return ChatModule_ChatMessageInitializeEx(self, NULL, true);
}

void ChatModule_ChatMessageFinalize(ChatModule_ChatMessage* self)
{
    ChatModule_ChatMessageFinalizeEx(self, NULL, true);
}

DDS_Boolean ChatModule_ChatMessageCopy(
    ChatModule_ChatMessage* dst,
    const ChatModule_ChatMessage* src)
{
    return ChatModule_ChatMessageCopyEx(dst, src, NULL);
}

void ChatModule_ChatMessageDestroySample(ZRMemPool* pool, ChatModule_ChatMessage* sample)
{
    if (sample == NULL) return;
    ChatModule_ChatMessageFinalizeEx(sample, pool, true);
    ZRDealloc(pool, sample);
}

ChatModule_ChatMessage* ChatModule_ChatMessageCreateSample(
    ZRMemPool* pool,
    DDS_Boolean allocMutable)
{
    ChatModule_ChatMessage* newSample = (ChatModule_ChatMessage*)ZRMalloc(pool, sizeof(ChatModule_ChatMessage));
    if (newSample == NULL)
    {
        printf("malloc for ChatModule_ChatMessage failed.");
        return NULL;
    }
    if (!ChatModule_ChatMessageInitializeEx(newSample, pool, allocMutable))
    {
        printf("initial Sample failed.");
        ChatModule_ChatMessageDestroySample(pool, newSample);
        return NULL;
    }
    return newSample;
}

DDS_ULong ChatModule_ChatMessageGetSerializedSampleMaxSize()
{
    return 1044;
}

DDS_ULong ChatModule_ChatMessageGetSerializedKeyMaxSize()
{
    return 1044;
}

DDS_Long ChatModule_ChatMessageGetKeyHash(
    const ChatModule_ChatMessage* sample,
    CDRSerializer* cdr,
    DDS_KeyHash_t* result)
{
    DDS_Long ret = ChatModule_ChatMessageSerializeKey(sample, cdr);
    if (ret < 0)
    {
        printf("serialize key failed.");
        *result = DDS_HANDLE_NIL_NATIVE;
        return -1;
    }
    ret = CDRSerializeGetKeyHash(cdr, result->value, true);
    if (ret < 0)
    {
        printf("get keyhash failed.");
        *result = DDS_HANDLE_NIL_NATIVE;
        return -1;
    }
    result->valid = true;
    return 0;
}

DDS_Boolean ChatModule_ChatMessageHasKey()
{
    return false;
}

TypeCodeHeader* ChatModule_ChatMessageGetInnerTypeCode()
{
    return ChatModule_ChatMessageGetTypeCode();
}

