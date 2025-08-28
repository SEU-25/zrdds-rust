/**
 * @file:       ZRDDSCWrapper.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef ZRDDSCWrapper_h__
#define ZRDDSCWrapper_h__

/**
 * @struct DDS_Condition
 *
 * @ingroup CInfrastruct
 *
 * @brief   ZRDDS�������ġ����ࡱ��
 *
 * @details ZRDDS���ṩ����-�ȴ�ģ��ʹ���û�����ʹ��ͬ���ȴ���ģʽ��ȡZRDDS�ײ�����ݣ�@ref waitset-introduction ������Ϊ���������Ļ��ࡣ
 */

typedef struct ConditionImpl DDS_Condition;

/**
 * @struct DDS_StatusCondition
 *
 * @ingroup CInfrastruct
 *
 * @brief   ʵ��״̬���������������ڻ�ȡʵ��״̬�ı䡣
 *
 * @details ���������ڵȴ�ʵ���м�ͨ��״̬�仯����ZRDDS���û�����ʵ��ʱ���Զ�������ʵ������ĸ�״̬���û�ͨ���ӿ�
 *          #DDS_Entity_get_statuscondition ������ȡ�ײ����á�
 */

typedef struct StatusConditionImpl DDS_StatusCondition;

/**
 * @struct DDS_GuardCondition
 *
 * @ingroup CInfrastruct
 *
 * @brief   �������������ֶ����Ƶȴ�������
 *
 * @details ������Ҫ�����ֶ����Ƶȴ����� #DDS_WaitSet_wait ��������������������������������ȫ���û����ơ�
 */

typedef struct GuardConditionImpl DDS_GuardCondition;

/**
 * @struct DDS_ReadCondition
 *
 * @ingroup CSubscription
 *
 * @brief   ���������ڱ�ʾZRDDS�еĶ�ȡ����@ref waitset-introduction ��
 *
 * @details ZRDDS�����ݶ���Ϊÿ���洢������������ά������״̬��
 *          - #DDS_SampleStateKind ;
 *          - #DDS_ViewStateKind ;
 *          - #DDS_InstanceStateKind ;
 *
 *          �����ȡ���� DDS_ReadCondition(sampleMask, viewMask, instanceMask)��������������ͬʱ������������������
 *          - #DDS_SampleStateKind ���� sampleMask ����ʾ��״̬�����У�
 *          - �� #DDS_ViewStateKind ���� viewMask ����ʾ��״̬�����У�
 *          - �� #DDS_InstanceStateKind ���� instanceMask ����ʾ��״̬�����У�
 *          ��ȡ��������ͬʱ��ʾ������״̬����ȡ������Ҫ�������ط�ʹ�ã�
 *          1. ��������-�ȴ�ģ��@ref waitset-introduction �У������ݶ����д��� DDS_ReadCondition ��ָ��״̬������
 *              �������ϲ�Ϊ��ʱ����������������
 *          2. ���� @ref read-take ϵ�з��������� sample_mask��view_mask��instance_mask��
 *              ���������ڶ�ȡ���ݶ����д��� DDS_ReadCondition ��ָ��״̬�������������ϡ�
 */

typedef struct ReadConditionImpl DDS_ReadCondition;

/**
 * @struct DDS_QueryCondition
 *
 * @ingroup CSubscription
 *
 * @brief   ��ѯ������
 *
 * @warning ZRDDS��ǰδʵ�ָù��ܡ�
 */

typedef struct QueryConditionImpl DDS_QueryCondition;

/**
 * @struct DDS_WaitSet
 *
 * @ingroup CInfrastructure
 *
 * @brief   �����ͱ�ʾ����-�ȴ��еĵȴ����� @ref waitset-introduction ��
 */

typedef struct WaitSetImpl DDS_WaitSet;

/**
 * @struct DDS_Entity
 *
 * @ingroup CInfrastruct
 *
 * @brief   ���������ڱ�ʾ����ʵ�壨@ref entity-introduction) ����������ߡ����⡢�����ߡ������ߡ����ݶ��ߡ�����д�ߵġ����ࡱ��
 */

typedef struct EntityImpl DDS_Entity;

/**
 * @struct DDS_DataReader
 *
 * @ingroup CSubscription
 *
 * @brief   ��ʾZRDDS�е����ݶ��ߡ�
 *
 * @details ���ݶ�����Ҫ����洢�ӷ����˻�ȡ���������Լ��ṩ�ӿڸ��ϲ�Ӧ�û�ȡ���յ������ݣ�
 *          ZRDDS�ṩǿ���Ͱ�ȫ�ӿڵ����ݶ��߽ӿڣ���ϸ�Ľӿ�˵���μ� ::FooDataReader ��
 */

typedef struct DataReaderImpl DDS_DataReader;

/**
 * @struct DDS_DataWriter
 *
 * @ingroup CPublication
 *
 * @brief   ��ʾZRDDS�е�����д�ߡ�
 *
 * @details ����д����Ҫ���𷢲����ݣ�ZRDDS�ṩǿ���Ͱ�ȫ�ӿڵ����ݶ��߽ӿڣ���ϸ�Ľӿ�˵���μ� ::FooDataWriter ��
 */

typedef struct DataWriterImpl DDS_DataWriter;

/**
 * @struct DDS_TopicDescription
 *
 * @ingroup	CTopic
 *
 * @brief	��������Ļ���������Ϣ��
 *
 * @details	��������ص���Ϣ����������һ���Ѵ��ڵ��������ͣ��������������� DDS_TypeSupport ����������
 * 			���������Լ������������� DDS_DomainParticipant ����Щ������Ϣ�����ڸ���Ķ����в���ͨ������Ķ����ȡ��
 */

typedef struct TopicDescriptionImpl DDS_TopicDescription;

/**
 * @struct DDS_Topic
 *
 * @ingroup CTopic
 *
 * @brief   �����Ƿ�������ģ�������ݻ����ĳ���
 *
 * @details ��������������ʶ��������������Ψһ������ͨ���������������ƶ������������ݵ����ݽṹ��
 *          ����д�������ݶ���ͨ�������������ͨ�š�
 */

typedef struct TopicImpl DDS_Topic;

/**
 * @struct DDS_ContentFilteredTopic
 *
 * @ingroup  CppTopic
 *
 * @brief   ����������ݹ��˵����⡣
 *
 * @details  �������ݵĹ��������Ǹ����ӵ�������󣬱������Ķ˲���Ҫ�����µ��������ݣ���ֻ��������һ�����������ݡ�
 *           �������ݹ��˵���������������ƶ��Ķ���Ҫ��������������Լ����������ڷ��͵��������������˹��ˣ���
 *           ͨ�����˱��ʽ�Լ����˲�������ʾ���Ķ���Ҫ�����ݡ����˱��ʽ��SQL����е�WHERE�����ƣ�
 *           �����˲������ṩֵ�����˱��ʽ�еĲ�����ʹ��%n��ʾ�����ṩ�Ĺ��˲����ĸ���������˱��ʽ��Ҫ��Ĳ���һ�¡�
 *           ���˱��ʽ�Լ����˲������﷨�μ� @ref expression-grammer ��
 *
 *           ע�⣺ ZRDDS�ð汾�Ĺ����ڶ��Ķ�Ӧ�ã��ʶ��������ݹ��˵����ⲻ���������������������ӽ��ն˵ĸ��أ�ÿ������
 *           ����ҪӦ���û����õĹ��˹��򣩡�
 */

typedef struct ContentFilteredTopicImpl DDS_ContentFilteredTopic;

/**
 * @struct DDS_Subscriber
 *
 * @ingroup  CSubscription
 *
 * @brief   ZRDDS�ṩ�Ķ����߽ӿڣ�Ӧ�ô��������߱�ʾ�������ָ�������ڻ�ȡ���ݣ�
 *
 * @details ��������Ҫ������¼������ܣ�
 *          - ʵ�幦�ܣ�
 *          - ��Ϊ���ݶ��ߵĹ�����
 *          - ͳһ�������ݶ��ߵ���������֪ͨ��
 */

typedef struct SubscriberImpl DDS_Subscriber;

/**
 * @struct DDS_Publisher
 *
 * @ingroup  CPublication
 *
 * @brief   ZRDDS�ṩ�ķ����߽ӿڣ�Ӧ�ô��������߱�ʾ��������ָ���������ṩ���ݣ�
 *
 * @details ��������Ҫ������¼������ܣ�
 *          - ʵ�幦�ܣ�
 *          - ��Ϊ����д�ߵĹ�����
 *          - ͳһ��������д�ߵ������������ͣ�
 */

typedef struct PublisherImpl DDS_Publisher;

/**
 * @struct DDS_DomainParticipant
 *
 * @ingroup CDomain
 *
 * @brief   ��������������ʵ�塣
 *
 * @details ���������Ҫ�ṩ���½ӿڹ��ܣ�
 *          - ʵ�幦�ܣ�
 *          - ��Ϊ����ʵ�幤����
 *              - ��Ϊ������ʵ�幤����
 *              - ��Ϊ������ʵ�幤����
 *              - ��Ϊ����ʵ�幤����
 *          - ����ʵ�����
 *          - ͨ�Ź���
 *          - ����Ϣ��ѯ��
 *          - ����Թ���
 */

typedef struct DomainParticipantImpl DDS_DomainParticipant;

/**
 * @struct DDS_DomainParticipantFactory
 *
 * @ingroup CDomain
 *
 * @brief   ������߹����ӿڵĺ��Ĺ��������Ǵ����Լ������������ʵ�壬����Ϊ�������ʵ��Ĺ�����
 */

typedef struct DomainParticipantFactoryImpl DDS_DomainParticipantFactory;

typedef struct ZRDynamicDataDataReader ZRDynamicDataDataReader;
typedef struct ZRDynamicDataTypeSupportImpl ZRDynamicDataTypeSupport;

typedef struct TypeCodeHeader TypeCode;
typedef struct TypeCodeFactoryImpl TypeCodeFactory;

typedef struct DataWriterListenerImpl DataWriterListenerImpl;
typedef struct DataWriterImpl DataWriterImpl;
typedef struct DataReaderImpl DataReaderImpl;

#endif /* ZRDDSCWrapper_h__ */
