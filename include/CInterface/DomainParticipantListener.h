/**
 * @file:       DomainParticipantListener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef DomainParticipantListener_h__
#define DomainParticipantListener_h__

#include "TopicListener.h"
#include "PublisherListener.h"
#include "SubscriberListener.h"

/**
 * @struct DDS_DomainParticipantListener
 *
 * @ingroup  CListener
 *
 * @brief   ������߼��������͡�
 */

typedef struct DDS_DomainParticipantListener
{
    /** @brief   �ó�Ա���ڼ����������������ʵ���״̬�� */
    DDS_TopicListener topiclistener;
    /** @brief   �ó�Ա���ڼ���������߷�������ʵ���״̬�� */
    DDS_PublisherListener publisherlistener;
    /** @brief   �ó�Ա���ڼ���������߶�������ʵ���״̬�� */
    DDS_SubscriberListener subscriberlistener;
}DDS_DomainParticipantListener ;

/**
 * @def DDS_DomainParticipantListener_initial(listener)
 *
 * @ingroup CListener
 *
 * @brief   ��ʼ��������߼���������Ϊ���г�ԱΪ�ա�
 *
 * @param   listener    ��Ҫ��ʼ����������߼���������
 */

#define DDS_DomainParticipantListener_initial(listener) memset(listener, 0, sizeof(DDS_DomainParticipantListener))

#endif /* DomainParticipantListener_h__*/
