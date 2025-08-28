/**
 * @file:       ZRDDSTypeSupport.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef ZRDDSTypeSupport_h__
#define ZRDDSTypeSupport_h__

#include "OsResource.h"
#include "ZRDDSCommon.h"
#include "ReturnCode_t.h"
#include "ZRDDSTypePlugin.h"
#include "ZRDDSCWrapper.h"

#ifdef __cplusplus
extern "C"
{
#endif

DCPSDLL extern DDS_ReturnCode_t DomainParticipantRegisterType(
    DDS_DomainParticipant* self,
    const DDS_Char* typeName,
    ZRDDSTypePlugin* typePlugin);

DCPSDLL extern DDS_ReturnCode_t DomainParticipantUnRegisterType(
    DDS_DomainParticipant* self,
    const DDS_Char* typeName);

typedef DDS_ReturnCode_t(*ZRDDSTypeSupportRegisterTypeFunc)(DDS_DomainParticipant* participant, const DDS_Char* typeName);
typedef DDS_ReturnCode_t(*ZRDDSTypeSupportUnRegisterTypeFunc)(DDS_DomainParticipant* participant, const DDS_Char* typeName);
typedef const DDS_Char*(*ZRDDSTypeSupportGetTypeNameFunc)();

/**
 * @typedef struct DDS_TypeSupport
 *
 * @brief   C�ӿ�����֧�ֽṹ������TypeSupport�ĺ���ָ�룬����C�ӿڴ�������ʱ�Զ�ע�����͡�
 */

typedef struct DDS_TypeSupport
{
    /** @brief   ע�����ͺ����� */
    ZRDDSTypeSupportRegisterTypeFunc register_type_func;
    /** @brief   ע�����ͺ����� */
    ZRDDSTypeSupportUnRegisterTypeFunc unregister_type_func;
    /** @brief   ��ȡ�������ƺ����� */
    ZRDDSTypeSupportGetTypeNameFunc get_typename_func;
}DDS_TypeSupport;

#ifdef __cplusplus
}
#endif

#define DDSTypeSupport(TTypeSupport, TType)                                                                         \
    DDS_ReturnCode_t TTypeSupport##_register_type(DDS_DomainParticipant* participant, const DDS_Char* typeName);    \
    DDS_ReturnCode_t TTypeSupport##_unregister_type(DDS_DomainParticipant* participant, const DDS_Char* typeName);  \
    const DDS_Char* TTypeSupport##_get_type_name();                                                                 \
    extern DDS_TypeSupport TTypeSupport##_instance

#define DDSInnerTypeSupport(TTypeSupport, TType)                                                                            \
    DCPSDLL DDS_ReturnCode_t TTypeSupport##_register_type(DDS_DomainParticipant* participant, const DDS_Char* typeName);    \
    DCPSDLL DDS_ReturnCode_t TTypeSupport##_unregister_type(DDS_DomainParticipant* participant, const DDS_Char* typeName);  \
    DCPSDLL const DDS_Char* TTypeSupport##_get_type_name();                                                                 \
    DCPSDLL extern DDS_TypeSupport TTypeSupport##_instance

#endif /* ZRDDSTypeSupport_h__*/
