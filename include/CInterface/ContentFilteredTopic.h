/**
 * @file:       ContentFilteredTopic.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef ContentFilteredTopic_h__
#define ContentFilteredTopic_h__

#include "OsResource.h"
#include "Topic.h"

#ifdef _ZRDDS_INCLUDE_CONTENTFILTER_TOPIC

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DDS_Topic* DDS_ContentFilterTopic_get_related_topic(DDS_ContentFilteredTopic* self);
 *
 * @ingroup  CTopic
 *
 * @brief   ��ȡ�û������ݹ�������Ļ������⣬��ʹ�� DDS_DomainParticipant_create_contentfilertopic ����ʱָ����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ָ��û������ݹ��˵���������Ļ������⡣
 */

DCPSDLL DDS_Topic* DDS_ContentFilterTopic_get_related_topic(
    DDS_ContentFilteredTopic* self);

/**
 * @fn  DDS_ReturnCode_t DDS_ContentFilteredTopic_get_expression_paramters(DDS_ContentFilteredTopic* self, DDS_StringSeq* para);
 *
 * @ingroup  CTopic
 *
 * @brief   �÷�����ȡ�����Ĺ��˲������ڴ����������ݹ��˵�����ʱ�������ͨ�� #DDS_ContentFilteredTopic_set_expression_paramters �������á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  para    ���ڲ��������ڱ���ͨ���м��ά���Ĺ��˲�����
 *
 * @return  ���µĿ��ܷ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ�ɹ���
 *          - #DDS_RETCODE_ERROR :δ֪�Ĵ����ʾʧ�ܣ����翽������ʧ�ܡ�
 */

DCPSDLL DDS_ReturnCode_t DDS_ContentFilteredTopic_get_expression_paramters(
    DDS_ContentFilteredTopic* self, 
    DDS_StringSeq* para);

/**
 * @fn  DDS_ReturnCode_t DDS_ContentFilteredTopic_set_expression_paramters(DDS_ContentFilteredTopic* self, const DDS_StringSeq* para);
 *
 * @ingroup  CTopic
 *
 * @brief   �÷��������������ù��˲�����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   para            ָ���µĹ��˲�����
 *
 * @return  ���µĿ��ܷ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ�ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ���˲�������������������˱��ʽ��ƥ�䣻
 *          - #DDS_RETCODE_ERROR :δ֪�Ĵ����ʾʧ�ܣ����翽������ʧ�ܡ�
 */

DCPSDLL DDS_ReturnCode_t DDS_ContentFilteredTopic_set_expression_paramters(
    DDS_ContentFilteredTopic* self, 
    const DDS_StringSeq* para);

/**
 * @fn  const DDS_Char* DDS_ContentFilteredTopic_get_filter_expression(DDS_ContentFilteredTopic* self);
 *
 * @ingroup  CTopic
 *
 * @brief   ��ȡ��ǰ�������ݹ��˵���������Ĺ��˱��ʽ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ���˱��ʽ�ַ�����
 */

DCPSDLL const DDS_Char* DDS_ContentFilteredTopic_get_filter_expression(
    DDS_ContentFilteredTopic* self);

#ifdef __cplusplus
}
#endif

#endif /* _ZRDDS_INCLUDE_CONTENTFILTER_TOPIC */

#endif /* ContentFilteredTopic_h__*/
