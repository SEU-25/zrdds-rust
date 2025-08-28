/**
 * @file:       Entity.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef Entity_h__
#define Entity_h__

#include "InstanceHandle_t.h"
#include "Condition.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_InstanceHandle_t DDS_Entity_get_instance_handle(const DDS_Entity* self);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡ��ʵ���Ψһ��ʶ��
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 *
 * @return  ʵ��Ψһ��ʶ��
 */

DCPSDLL DDS_InstanceHandle_t DDS_Entity_get_instance_handle(
    const DDS_Entity* self);

/**
 * @fn  DCPSDLL DDS_StatusCondition* DDS_Entity_get_statuscondition(DDS_Entity* self);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ���ظ�ʵ�������״̬������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 *
 * @return  �ײ��Զ�������״̬������
 */

DCPSDLL DDS_StatusCondition* DDS_Entity_get_statuscondition(
    DDS_Entity* self);

/**
 * @fn  DCPSDLL DDS_UShort DDS_Entity_get_status_changes(DDS_Entity* self);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡ��ʵ�����һ�λ�ȡ����״̬���״̬�仯��
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 *
 * @return  ����״̬�ĸı����롣
 */

DCPSDLL DDS_UShort DDS_Entity_get_status_changes(
    DDS_Entity* self);

#ifdef __cplusplus
}
#endif

#endif /* Entity_h__*/
