/**
 * @file:       TypeCode.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef TypeCode_h__
#define TypeCode_h__

#include "OsResource.h"
#include "TypeCodeKind.h"
#include "ZRDDSCommon.h"
#include "ZRDDSCWrapper.h"

#ifdef _ZRDDS_INCLUDE_TYPECODE

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  TCTypeKind TypeCodeGetKind(const TypeCode* self);
 *
 * @brief   ��ȡTypeCode��ʾ������
 *
 * @author  Hzy
 * @date    2016/7/5
 *
 * @param   self    ָ��Ŀ��TypeCode��
 *
 * @return  TC�����͡�
 */

DCPSDLL extern TCTypeKind TypeCodeGetKind(const TypeCode* self);

/**
 * @fn  ZR_BOOLEAN TypeCodeCompare(const TypeCode* self TypeCode1, const TypeCode* right TypeCode2);
 *
 * @brief   �Ƚ�����TypeCode�Ƿ�ָ���ʾͬһ���ṹ.
 *
 * @author  Hzy
 * @date    2016/7/5
 *
 * @param   TypeCode1   ��һ��TypeCode��
 * @param   TypeCode2   �ڶ���TypeCode��
 *
 * @return  true��ʾ��ȣ�false��ʾ����ȡ�
 */

DCPSDLL extern ZR_BOOLEAN TypeCodeCompare(const TypeCode* typeCode1, const TypeCode* typeCode2);

/**
 * @fn  const ZR_INT8* TypeCodeGetName(const TypeCode* self);
 *
 * @brief   ��ȡTypeCode�����ƣ�ֻ֧��union/struct/enum���͡�
 *
 * @author  Hzy
 * @date    2016/7/5
 *
 * @param   self    ָ��Ŀ��TypeCode
 *
 * @return  NULL��ʾʧ�ܣ������ʾ���ơ�
 */

DCPSDLL extern const ZR_INT8* TypeCodeGetName(const TypeCode* self);

/**
 * @fn  ZR_INT32 TypeCodeGetMemberCount(const TypeCode* self);
 *
 * @brief   ��ȡTypeCode��ʾ�����ͳ�Ա������ֻ֧��union/struct/enum���͡�
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ��TypeCode
 *
 * @return  С��0��ʾʧ�ܣ�����Ϊ��Чֵ
 */

DCPSDLL extern ZR_INT32 TypeCodeGetMemberCount(const TypeCode* self);

/**
 * @fn  const ZR_INT8* TypeCodeGetMemberName(const TypeCode* self, ZR_UINT32 index);
 *
 * @brief   ��ȡָ���±��Ա������.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 * @param   index   Ŀ���Ա�±ꡣ
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч�ĳ�Ա����ָ�롣
 */

DCPSDLL extern const ZR_INT8* TypeCodeGetMemberName(const TypeCode* self, ZR_UINT32 index);

/**
 * @fn  DCPSDLL const TypeCodeHeader* TypeCodeGetBaseType(const TypeCodeHeader* self);
 *
 * @brief   ��ȡTypeCode���ṹ����������VALUE_TYPE.
 *
 * @author  Hzy
 * @date    2016/12/13
 *
 * @param   self    ָ��VALUE_TYPE.
 *
 * @return  NULL��ʾ��ȡʧ�ܣ�����ִ�и��ṹ���ࡣ
 */

DCPSDLL extern const TypeCodeHeader* TypeCodeGetBaseType(const TypeCodeHeader* self);

/**
 * @fn  DCPSDLL extern TypeCode* TypeCodeGetElementType(const TypeCode* self);
 *
 * @brief   Type code get element type.
 *
 * @author  Rainnus
 * @date    2016/9/27
 *
 * @param   self    The self.
 *
 * @return  null if it fails, else a TypeCodeHeader*.
 */

DCPSDLL extern TypeCode* TypeCodeGetElementType(const TypeCode* self);

/**
 * @fn  ZR_INT32 TypeCodeGetIndexByName(const TypeCode* self, const ZR_INT8* name);
 *
 * @brief   ͨ����Ա���ƻ�ȡ�±��0��ʼ.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 * @param   name    ��Ա���ơ�
 *
 * @return  С�����ʾʧ�ܣ�����Ϊ��Ч���±ꡣ
 */

DCPSDLL extern ZR_INT32 TypeCodeGetIndexByName(const TypeCode* self, const ZR_INT8* name);

/**
 * @fn  TypeCode*TypeCodeGetMemberType(const TypeCode* self, ZR_UINT32 index);
 *
 * @brief   ��ȡָ���±��Ա��TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 * @param   index   ��ʾ�±ꡣ
 */

DCPSDLL extern TypeCode*TypeCodeGetMemberType(const TypeCode* self, ZR_UINT32 index);

/**
 * @fn  ZR_INT32 TypeCodeGetLabelCount(const TypeCode* self, ZR_UINT32 index);
 *
 * @brief   ��ȡUnion����ָ���ĳ�Ա��label��������Ч���� - 1.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 * @param   index   ָ���±ꡣ
 *
 * @return  A ZR_INT32.
 */

DCPSDLL extern ZR_INT32 TypeCodeGetLabelCount(const TypeCode* self, ZR_UINT32 index);

/**
 * @fn  ZR_INT32 TypeCodeGetLabel(const TypeCode* self, ZR_UINT32 memberIdx, ZR_UINT32 labelIdx, TypeCodeExceptionCode* expCode);
 *
 * @brief   ��ȡUnion��Ա��label.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self            ָ��Ŀ�ꡣ
 * @param   memberIdx       ��Ա�±ꡣ
 * @param   labelIdx        ��ǩ�±ꡣ
 * @param [in,out]  expCode �����Ƿ���Ч��������Ϊ������Ч��Ҫ������Чֵ��
 *
 * @return  ���expCode��Ч���򷵻�ֵΪlabel������÷���ֵ��Ч��
 */

DCPSDLL extern ZR_INT32 TypeCodeGetLabel(const TypeCode* self,
    ZR_UINT32 memberIdx,
    ZR_UINT32 labelIdx,
    TypeCodeExceptionCode* expCode);

/**
 * @fn  ZR_INT32 TypeCodeGetDefaultIndex(const TypeCode* self);
 *
 * @brief   ��ȡUnion���͵�Ĭ���±�.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 *
 * @return  -1��ʾʧ�ܣ�����Ϊ��Ч���±ꡣ
 */

DCPSDLL extern ZR_INT32 TypeCodeGetDefaultIndex(const TypeCode* self);

/**
 * @fn  ZR_INT32 TypeCodeGetEnumVal(const TypeCode* self, ZR_UINT32 memberIdx, TypeCodeExceptionCode* expCode);
 *
 * @brief   ��ȡö��ֵ��ֵ��
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self            ָ��Ŀ�ꡣ
 * @param   memberIdx       ��Ա�±ꡣ
 * @param [in,out]  expCode �����Ƿ���Ч��������Ϊ������Ч��Ҫ������Чֵ��
 *
 * @return  ���expCode��Ч���򷵻�ֵΪö�ٵ�ֵ������÷���ֵ��Ч��
 */

DCPSDLL extern ZR_INT32 TypeCodeGetEnumVal(const TypeCode* self,
    ZR_UINT32 memberIdx,
    TypeCodeExceptionCode* expCode);

/**
 * @fn  DCPSDLL const ZR_INT8* TypeCodeGetEnumString(const TypeCode* self, ZR_UINT32 memberIdx, TypeCodeExceptionCode* expCode);
 *
 * @brief   ��ȡö�ٳ�Ա��Ӧ���ַ�����
 *
 * @author  Hzy
 * @date    2018/3/16
 *
 * @param   self            ָ��TypeCode.
 * @param   enumVal         ö��ֵ��
 * @param [in,out]  expCode ���expCode��Ч���򷵻�ֵΪö�ٵ�ֵ������÷���ֵ��Ч��
 *
 * @return  NULL��ʾʧ�ܣ������ʾö���ַ���ֵ��
 */

DCPSDLL extern const ZR_INT8* TypeCodeGetEnumString(const TypeCode* self,
    ZR_UINT32 enumVal,
    TypeCodeExceptionCode* expCode);

/**
 * @fn  ZR_BOOLEAN TypeCodeIsMemberKey(const TypeCode* self, ZR_UINT32 index, TypeCodeExceptionCode* expCode);
 *
 * @brief   ��ȡָ���ṹ���Ա�Ƿ���key��.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self            ָ��Ŀ�ꡣ
 * @param   index           ָ���Ա�±ꡣ
 * @param [in,out]  expCode �����Ƿ���Ч��������Ϊ������Ч��Ҫ������Чֵ��
 *
 * @return  ���expCode��Ч���򷵻�ֵΪʱ����key�򣬷���÷���ֵ��Ч��
 */

DCPSDLL extern ZR_BOOLEAN TypeCodeIsMemberKey(const TypeCode* self,
    ZR_UINT32 index,
    TypeCodeExceptionCode* expCode);

/**
 * @fn  ZR_INT32 TypeCodeGetArrayDimensionCount(const TypeCode* self);
 *
 * @brief   ��ȡ�������͵�ά�ȡ�
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 *
 * @return  С��0��ʾ��Ч,�����ʾά�ȡ�
 */

DCPSDLL extern ZR_INT32 TypeCodeGetArrayDimensionCount(const TypeCode* self);

/**
 * @fn  ZR_INT32 TypeCodeGetArrayDimension(const TypeCode* self, ZR_INT32 index);
 *
 * @brief   ��ȡָ���±�ά�ȵ�ά��.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 * @param   index   ָ��ά�ȡ�
 *
 * @return  С�����ʾʧ�ܣ�����Ϊ��Ч��ά����
 */

DCPSDLL extern ZR_INT32 TypeCodeGetArrayDimension(const TypeCode* self, ZR_UINT32 index);

/**
 * @fn  DCPSDLL ZR_INT32 TypeCodeGetArrayElementCount(const TypeCodeHeader* self);
 *
 * @brief   ��ȡ���͵���󳤶ȣ�֧��string/sequence��
 *
 * @author  Hzy 
 * @date    2018/2/6
 *
 * @param   self    ָ�����
 *
 * @return  ��󳤶ȡ�
 */

DCPSDLL extern ZR_INT32 TypeCodeGetMaxLength(const TypeCodeHeader* self);

/**
 * @fn  DCPSDLL extern ZR_INT32 TypeCodeGetArrayElementCount(const TypeCode* self);
 *
 * @brief   ��ȡ����Ԫ�ظ�����
 *
 * @author  Rainnus
 * @date    2016/10/25
 *
 * @param   self    ָ�����
 *
 * @return  ����Ԫ�ظ�����
 */

DCPSDLL extern ZR_INT32 TypeCodeGetArrayElementCount(const TypeCode* self);

/**
 * @fn  ZR_INT32 TypeCodeAddMemberToEnum(TypeCode* self, ZR_UINT32 index, const ZR_INT8* name, ZR_UINT32 value);
 *
 * @brief   ��ö��������ӳ�Ա
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   index           ָ����ӵ�λ�ã����ڵ�ǰ��Ա�����������β����
 * @param   name            ��Ա���ơ�
 * @param   value           ��ӵĳ�Աֵ��
 *
 * @return  ����0��ʾ��Ӻ�ö�ٳ�Ա������С��0��ʾʧ��.
 */

DCPSDLL extern ZR_INT32 TypeCodeAddMemberToEnum(TypeCode* self,
    ZR_UINT32 index,
    const ZR_INT8* name,
    ZR_UINT32 value);

/**
 * @fn  ZR_INT32 TypeCodeAddMemberToUnion(TypeCode* self, ZR_UINT32 index, const ZR_INT8* name, ZR_UINT32 labelCount, ZR_UINT32* labels, const TypeCode* tc);
 *
 * @brief   ������������ӳ�Ա
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   index           ָ����ӵ�λ�ã����ڵ�ǰ��Ա�����������β����
 * @param   name            ��Ա�����ơ�
 * @param   labelCount      �ó�Ա��ǩ������
 * @param [in,out]  labels  �ó�Ա�ı�ǩ���顣
 * @param   tc              �ó�Ա�����͡�
 *
 * @return  ������Ӻ����ϳ�Ա������С��0��ʾʧ�ܡ�
 */

DCPSDLL extern ZR_INT32 TypeCodeAddMemberToUnion(TypeCode* self, ZR_UINT32 index, const ZR_INT8* name, ZR_UINT32 labelCount, ZR_UINT32* labels, const TypeCode* tc);

/**
 * @fn  DCPSDLL ZR_INT32 TypeCodeAddMemberToStruct(TypeCodeHeader* self, ZR_UINT32 index, ZR_UINT32 memberId, const ZR_INT8* name, const TypeCodeHeader* tc, ZR_BOOLEAN isKey, ZR_BOOLEAN isOption);
 *
 * @brief   ��ṹ��������ӳ�Ա.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   index           ָ����ӵ�λ�ã����ڵ�ǰ��Ա�����������β����
 * @param   memberId        ָ���ó�Ա��Id, -1��ʾ���±�һ�¡�
 * @param   name            ��Ա�����ơ�
 * @param   tc              ��Ա���͡�
 * @param   isKey           �ó�Ա�Ƿ���key��
 * @param   isOption        �ó�Ա�Ƿ��ǿ�ѡ�ġ�
 *
 * @return  ������Ӻ�ṹ���Ա������С��0��ʾʧ�ܡ�
 */

DCPSDLL extern ZR_INT32 TypeCodeAddMemberToStruct(
    TypeCode* self, 
    ZR_UINT32 index, 
    ZR_UINT32 memberId, 
    const ZR_INT8* name, 
    const TypeCode* tc, 
    ZR_BOOLEAN isKey, 
    ZR_BOOLEAN isOption);

/**
 * @fn  DCPSDLL ZR_INT32 TypeCodeAddMemberToValueType(TypeCodeHeader* self, ZR_UINT32 index, ZR_UINT32 memberId, const ZR_INT8* name, const TypeCodeHeader* tc, ZR_BOOLEAN isKey, ZR_BOOLEAN isOption);
 *
 * @brief   ��ValueType������ӳ�Ա.
 *
 * @author  Rainnus
 * @date    2016/10/21
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   index           ָ����ӵ�λ�ã����ڵ�ǰ��Ա�����������β����
 * @param   memberId        ָ���ó�Ա��Id, -1��ʾ���±�һ�¡�
 * @param   name            ��Ա�����ơ�
 * @param   tc              ��Ա���͡�
 * @param   isKey           �ó�Ա�Ƿ���key��
 * @param   isOption        �ó�Ա�Ƿ��ǿ�ѡ�ġ�
 *
 * @return  ������Ӻ�ValueType��Ա������С��0��ʾʧ�ܡ�
 */

DCPSDLL extern ZR_INT32 TypeCodeAddMemberToValueType(
    TypeCode* self,
    ZR_UINT32 index,
    ZR_UINT32 memberId,
    const ZR_INT8* name,
    const TypeCode* tc,
    ZR_BOOLEAN isKey,
    ZR_BOOLEAN isOption);
/**
 * @fn  ZR_INT8* TypeCodeGetTypePrintableString(const TypeCodeHeader* self);
 *
 * @brief   ��ȡ�ȼ۵�IDL�ļ����ݵĻ��������˷��������سɹ�����������TypeCodeReleasePrintableString�����ͷſռ䡣
 *
 * @author  Hzy
 * @date    2016/12/13
 *
 * @param   self    ָ����TypeCode��
 *
 * @return  NULL��ȡʧ�ܣ�����ָ�����ݡ�
 */

DCPSDLL extern ZR_INT8* TypeCodeGetTypePrintableString(const TypeCodeHeader* self);

/**
 * @fn  DCPSDLL void TypeCodeReleasePrintableString(ZR_INT8* buffer);
 *
 * @brief   �ͷ���TypeCodeGetTypePrintableString�ײ㴴���Ŀռ䡣
 *
 * @author  Hzy
 * @date    2016/12/13
 *
 * @param [in,out]  buffer  ָ����TypeCodeGetTypePrintableString���صĿռ䡣
 */

DCPSDLL extern void TypeCodeReleasePrintableString(ZR_INT8* buffer);
/**
 * @fn  ZR_INT32 TypeCodePrintIDL(const TypeCodeHeader* self);
 *
 * @brief   ��ӡָ����TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param   self    ָ��Ŀ�ꡣ
 *
 * @return  С�����ʾ��ӡ����0��ʾ��ӡ�ɹ���
 */

DCPSDLL extern ZR_INT32 TypeCodePrintIDL(const TypeCode* self);

/**
 * @fn  TypeCodeFactory* TypeCodeFactoryGetInstance();
 *
 * @brief   ��ȡTypeCodeFactory�ĵ���.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @return  NULL��ʾʧ�ܣ�����ɹ���
 */

DCPSDLL extern TypeCodeFactory* TypeCodeFactoryGetInstance();

/**
 * @fn  ZR_BOOLEAN TypeCodeFactoryFinalize();
 *
 * @brief   ����ȫ��TypeCode������
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @return  true��ʾ�ɹ���false��ʾʧ�ܡ�
 */

DCPSDLL extern ZR_BOOLEAN TypeCodeFactoryFinalize();

/**
 * @fn  TypeCode* TypeCodeFactoryGetPrimitiveTC(TypeCodeFactory* self, TCTypeKind kind);
 *
 * @brief   ��ȡ�����������͵�TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   kind            ָ�������������͡�
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryGetPrimitiveTC(TypeCodeFactory* self, TCTypeKind kind);

/**
 * @fn  DCPSDLL TypeCodeHeader* TypeCodeFactoryCreateStructTC(TypeCodeFactoryImpl* self, const ZR_INT8* name, ExtensibilityKind kind);
 *
 * @brief   �����ṹ��TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   name            �ṹ������ơ�
 * @param   kind            �ýṹ���extensibility���ԡ�
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateStructTC(
    TypeCodeFactory* self, 
    const ZR_INT8* name, 
    ExtensibilityKind kind);

/**
 * @fn  DCPSDLL TypeCodeHeader* TypeCodeFactoryCreateValueTypeTC(TypeCodeFactoryImpl* self, const ZR_INT8* name, TypeCodeModifierKind modifierKind, ExtensibilityKind kind, const TypeCodeHeader *baseTC);
 *
 * @brief   Type code factory create value type tc.
 *
 * @author  Rainnus
 * @date    2016/10/21
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   name            valueType���ơ�
 * @param   modifierKind    modifier�����͡�
 * @param   kind            extensibility���͡�
 * @param   baseTC          �̳е����͡�
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateValueTypeTC(
    TypeCodeFactory* self, 
    const ZR_INT8* name, 
    TypeCodeModifierKind modifierKind, 
    ExtensibilityKind kind, 
    const TypeCode*baseTC);

/**
 * @fn  DCPSDLL TypeCodeHeader* TypeCodeFactoryCreateEnumTC( TypeCodeFactoryImpl* self, const ZR_INT8* name, ZR_UINT32 bitBound, ExtensibilityKind kind);
 *
 * @brief   ����ö������TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   name            ö�����͵����ơ�
 * @param   bitBound        λ���С��
 * @param   kind            Extenbility����ֵ��
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateEnumTC(
    TypeCodeFactory* self, 
    const ZR_INT8* name,
    ZR_UINT32 bitBound,
    ExtensibilityKind kind);

/**
 * @fn  TypeCode* TypeCodeFactoryCreateUnionTC(TypeCodeFactory* self, const ZR_INT8* name, TypeCode* switchTC, ZR_UINT32 defaultIdx);
 *
 * @brief   ������������TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   name                �������͵����ơ�
 * @param [in,out]  switchTC    ������TC��
 * @param   defaultIdx          Ĭ�ϵĳ�Ա�±ꡣ
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateUnionTC(TypeCodeFactory* self, const ZR_INT8* name, const TypeCode* switchTC, ZR_UINT32 defaultIdx);

/**
 * @fn  TypeCode* TypeCodeFactoryCreateStringTC(TypeCodeFactory* self, const ZR_UINT32 length);
 *
 * @brief   ����String����TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   length          string�ĳ��ȡ�
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateStringTC(TypeCodeFactory* self, const ZR_UINT32 length);

/**
 * @fn  TypeCode* TypeCodeFactoryCreateSequenceTC(TypeCodeFactory* self, const ZR_UINT32 maxLength, const TypeCode* tc);
 *
 * @brief   ����Sequence����TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   maxLength          Sequence����󳤶ȡ�
 * @param   tc              Ԫ�ص�TypeCode��
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateSequenceTC(TypeCodeFactory* self, const ZR_UINT32 maxLength, const TypeCode* tc);

/**
 * @fn  TypeCode* TypeCodeFactoryCreateArrayTC(TypeCodeFactory* self, const ZR_UINT32 dimensionCount, ZR_UINT* dimensions, const TypeCode* tc);
 *
 * @brief   ������������TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   dimensionCount      ����ά�ȡ�
 * @param [in,out]  dimensions  ����ά�ȵ�ά����
 * @param   tc                  Ԫ�ص�TypeCode��
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCreateArrayTC(TypeCodeFactory* self, const ZR_UINT32 dimensionCount, const ZR_UINT32* dimensions, const TypeCode* tc);

/**
 * @fn  TypeCode* TypeCodeFactoryCloneTC (TypeCodeFactory* self, const TypeCode* tc);
 *
 * @brief   ����һ��TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   tc              ԴTypeCode��
 *
 * @return  NULL��ʾʧ�ܣ�����Ϊ��Ч��TypeCode��
 */

DCPSDLL extern TypeCode* TypeCodeFactoryCloneTC(TypeCodeFactory* self, const TypeCode* tc);

/**
 * @fn  ZR_BOOLEAN TypeCodeFactoryDeleteTC(TypeCodeFactory* self, TypeCode* tc);
 *
 * @brief   ɾ��һ��TypeCode.
 *
 * @author  Hzy
 * @date    2016/7/6
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  tc      ɾ����Ŀ�ꡣ
 *
 * @return  true��ʾɾ���ɹ���false��ʾɾ��ʧ�ܡ�
 */

DCPSDLL extern ZR_BOOLEAN TypeCodeFactoryDeleteTC(TypeCodeFactory* self, TypeCode* tc);

#ifdef __cplusplus
}
#endif

#endif /* _ZRDDS_INCLUDE_TYPECODE */

#endif /* TypeCode_h__*/
