// Code generated by Prisma CRDT Generator. DO NOT EDIT

use crate::prisma;

pub async fn new_client(
	prisma_client: crate::prisma::PrismaClient,
	node_id: Vec<u8>,
	node_local_id: i32,
) -> (
	_prisma::PrismaCRDTClient,
	tokio::sync::mpsc::Receiver<prisma_crdt::CRDTOperation>,
) {
	let (tx, rx) = tokio::sync::mpsc::channel(64);

	let crdt_client = _prisma::PrismaCRDTClient::_new(prisma_client, (node_id, node_local_id), tx);
	(crdt_client, rx)
}
pub mod location {
	use super::*;
	pub use crate::prisma::location::*;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum SetParam {
		SetLocalId(i32),
		SetId(Vec<u8>),
		SetNodeId(i32),
		SetName(String),
	}

	impl Into<prisma::location::SetParam> for SetParam {
		fn into(self) -> prisma::location::SetParam {
			match self {
				SetParam::SetId(v) => prisma::location::SetParam::SetId(v),
				SetParam::SetLocalId(v) => prisma::location::SetParam::SetLocalId(v),
				SetParam::SetNodeId(v) => prisma::location::SetParam::SetNodeId(v),
				SetParam::SetName(v) => prisma::location::SetParam::SetName(v),
			}
		}
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum CRDTSetParam {
		SetLocalId(i32),
		SetId(Vec<u8>),
		SetNodeId(Vec<u8>),
		SetName(String),
	}

	impl Into<prisma::location::SetParam> for CRDTSetParam {
		fn into(self) -> prisma::location::SetParam {
			match self {
				CRDTSetParam::SetId(v) => prisma::location::id::set(v),
				CRDTSetParam::SetLocalId(v) => prisma::location::local_id::set(v),
				CRDTSetParam::SetNodeId(v) => {
					prisma::location::node::link(prisma::node::id::equals(v))
				}
				CRDTSetParam::SetName(v) => prisma::location::name::set(v),
			}
		}
	}

	pub struct Create<'a> {
		client: &'a _prisma::PrismaCRDTClient,
		set_params: CreateSetParams,
		with_params: Vec<crate::prisma::location::WithParam>,
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub struct CreateSetParams {
		id: Vec<u8>,
		name: String,
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		_params: Vec<SetParam>,
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub struct CRDTCreateParams {
		pub id: Vec<u8>,
		pub name: String,
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		pub _params: Vec<CRDTSetParam>,
	}

	impl<'a> Create<'a> {
		pub fn with(mut self, param: impl Into<crate::prisma::location::WithParam>) -> Self {
			self.with_params.push(param.into());
			self
		}

		pub async fn exec(
			self,
		) -> Result<crate::prisma::location::Data, crate::prisma::QueryError> {
			let Self {
				client,
				set_params,
				with_params,
			} = self;

			// THIS SHOULD BE IN A TRANSACTION

			let CreateSetParams {
				id: field_0,
				name: field_2,
				_params,
			} = set_params.clone();

			let res = client
				.client
				.location()
				.create(
					field_0,
					crate::prisma::node::local_id::equals(self.client.node_local_id),
					field_2,
					_params.into_iter().map(Into::into).collect::<Vec<_>>(),
				)
				.exec()
				.await;

			let set_params_map = match serde_json::to_value(set_params).unwrap() {
				serde_json::Value::Object(m) => m,
				_ => unreachable!(),
			};

			client
				._create_operation(::prisma_crdt::CRDTOperationType::owned(
					"Location",
					vec![prisma_crdt::OwnedOperationData::Create(set_params_map)],
				))
				.await;

			res
		}
	}

	pub struct Actions<'a> {
		pub(super) client: &'a _prisma::PrismaCRDTClient,
	}

	impl<'a> Actions<'a> {
		pub fn create(&self, id: Vec<u8>, name: String, mut _params: Vec<SetParam>) -> Create {
			Create {
				client: self.client,
				set_params: CreateSetParams { id, name, _params },
				with_params: vec![],
			}
		}

		pub fn find_unique(
			&self,
			param: crate::prisma::location::UniqueWhereParam,
		) -> crate::prisma::location::FindUnique {
			self.client.client.location().find_unique(param)
		}

		pub fn find_many(
			&self,
			params: Vec<crate::prisma::location::WhereParam>,
		) -> crate::prisma::location::FindMany {
			self.client.client.location().find_many(params)
		}
	}
}
pub mod file_path {
	use super::*;
	pub use crate::prisma::file_path::*;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum SetParam {
		SetName(String),
		SetFileId(Option<i32>),
		SetParentId(Option<i32>),
	}

	impl From<crate::prisma::file_path::name::Set> for SetParam {
		fn from(v: crate::prisma::file_path::name::Set) -> Self {
			SetParam::SetName(v.0)
		}
	}

	impl From<crate::prisma::file_path::file_id::Set> for SetParam {
		fn from(v: crate::prisma::file_path::file_id::Set) -> Self {
			SetParam::SetFileId(v.0)
		}
	}

	impl From<crate::prisma::file_path::parent_id::Set> for SetParam {
		fn from(v: crate::prisma::file_path::parent_id::Set) -> Self {
			SetParam::SetParentId(v.0)
		}
	}

	impl SetParam {
		fn into_crdt(self) -> CRDTSetParam {
			match self {
				SetParam::SetName(name) => CRDTSetParam::SetName(name),
				SetParam::SetFileId(file_id) => CRDTSetParam::SetFileId(file_id),
				SetParam::SetParentId(parent_id) => CRDTSetParam::SetParentId(parent_id),
			}
		}
	}

	impl Into<prisma::file_path::SetParam> for SetParam {
		fn into(self) -> prisma::file_path::SetParam {
			match self {
				Self::SetName(v) => prisma::file_path::name::set(v),
				Self::SetFileId(id) => prisma::file_path::SetParam::SetFileId(id),
				Self::SetParentId(id) => prisma::file_path::SetParam::SetParentId(id),
			}
		}
	}

	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
	pub enum CRDTSetParam {
		#[serde(rename = "name")]
		SetName(String),
		#[serde(rename = "file_id")]
		SetFileId(Option<i32>), // An owned record can only have intra-table relations to records with the same owner
		#[serde(rename = "parent_id")]
		SetParentId(Option<i32>),
	}

	impl Into<prisma::file_path::SetParam> for CRDTSetParam {
		fn into(self) -> prisma::file_path::SetParam {
			match self {
				Self::SetName(v) => prisma::file_path::name::set(v),
				Self::SetFileId(v) => prisma::file_path::file_id::set(v),
				Self::SetParentId(v) => prisma::file_path::parent_id::set(v),
			}
		}
	}

	pub struct Create<'a> {
		client: &'a _prisma::PrismaCRDTClient,
		set_params: CreateParams,
		with_params: Vec<crate::prisma::file_path::WithParam>,
	}

	#[derive(Clone)]
	pub(super) struct CreateParams {
		id: i32,
		location_id: i32,
		name: String,
		_params: Vec<SetParam>,
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub(super) struct CRDTCreateParams {
		pub id: i32,
		pub location_id: Vec<u8>,
		pub name: String,
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		pub _params: Vec<CRDTSetParam>,
	}

	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
	pub(super) struct CRDTUpdateParams {
		pub id: i32,
		pub location_id: Vec<u8>,
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		pub _params: Vec<CRDTSetParam>,
	}

	impl<'a> Create<'a> {
		pub fn with(mut self, param: impl Into<crate::prisma::file_path::WithParam>) -> Self {
			self.with_params.push(param.into());
			self
		}

		pub async fn exec(
			self,
		) -> Result<crate::prisma::file_path::Data, crate::prisma::QueryError> {
			let Self {
				client, set_params, ..
			} = self;

			// THIS SHOULD BE IN A TRANSACTION

			let CreateParams {
				id: field_0,
				location_id: field_1,
				name: field_2,
				_params,
			} = set_params.clone();

			let res = client
				.client
				.file_path()
				.create(
					field_0,
					crate::prisma::location::local_id::equals(field_1),
					field_2,
					_params.into_iter().map(Into::into).collect::<Vec<_>>(),
				)
				.with(crate::prisma::file_path::location::fetch())
				.exec()
				.await?;

			let set_params_map = {
				let CreateParams {
					id: field_0,
					location_id: field_1,
					name: field_2,
					_params,
				} = set_params;

				match serde_json::to_value(CRDTCreateParams {
					id: field_0,
					location_id: res.location().unwrap().id.clone(),
					name: field_2,
					_params: _params.into_iter().map(SetParam::into_crdt).collect(),
				})
				.unwrap()
				{
					serde_json::Value::Object(m) => m,
					_ => unreachable!(),
				}
			};

			client
				._create_operation(prisma_crdt::CRDTOperationType::owned(
					"FilePath",
					vec![prisma_crdt::OwnedOperationData::Create(set_params_map)],
				))
				.await;

			Ok(res)
		}
	}

	pub struct Update<'a> {
		client: &'a _prisma::PrismaCRDTClient,
		where_param: crate::prisma::file_path::UniqueWhereParam,
		set_params: Vec<SetParam>,
	}

	impl<'a> Update<'a> {
		pub async fn exec(
			self,
		) -> Result<Option<crate::prisma::file_path::Data>, crate::prisma::QueryError> {
			let Self {
				client,
				set_params,
				where_param,
			} = self;

			let res = client
				.client
				.file_path()
				.update(
					where_param,
					set_params.clone().into_iter().map(Into::into).collect(),
				)
				.with(crate::prisma::file_path::location::fetch())
				.exec()
				.await?;

			Ok(match res {
				Some(data) => {
					let crate::prisma::file_path::Data {
						location: field_0,
						id: field_1,
						..
					} = data.clone();

					let update_params_map = match ::serde_json::to_value(CRDTUpdateParams {
						location_id: field_0.unwrap().id,
						id: field_1,
						_params: set_params.into_iter().map(SetParam::into_crdt).collect(),
					})
					.unwrap()
					{
						serde_json::Value::Object(m) => m,
						_ => unreachable!(),
					};

					client
						._create_operation(::prisma_crdt::CRDTOperationType::owned(
							"FilePath",
							vec![::prisma_crdt::OwnedOperationData::Update(update_params_map)],
						))
						.await;

					Some(data)
				}
				None => None,
			})
		}
	}

	pub struct Actions<'a> {
		pub(super) client: &'a _prisma::PrismaCRDTClient,
	}

	impl<'a> Actions<'a> {
		pub fn create(
			self,
			id: i32,
			location_id: i32,
			name: String,
			mut _params: Vec<SetParam>,
		) -> Create<'a> {
			Create {
				client: self.client,
				set_params: CreateParams {
					id,
					location_id,
					name,
					_params,
				},
				with_params: vec![],
			}
		}

		pub fn find_unique(
			self,
			param: crate::prisma::file_path::UniqueWhereParam,
		) -> crate::prisma::file_path::FindUnique<'a> {
			self.client.client.file_path().find_unique(param)
		}

		pub fn find_many(
			self,
			params: Vec<crate::prisma::file_path::WhereParam>,
		) -> crate::prisma::file_path::FindMany<'a> {
			self.client.client.file_path().find_many(params)
		}

		pub fn update(
			self,
			_where: crate::prisma::file_path::UniqueWhereParam,
			set_params: Vec<SetParam>,
		) -> Update<'a> {
			Update {
				client: self.client,
				where_param: _where,
				set_params,
			}
		}
	}
}
pub mod file {
	pub use crate::prisma::file::*;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum SetParam {
		SetLocalId(i32),
		SetCasId(Vec<u8>),
		SetSizeInBytes(i32),
	}

	impl SetParam {
		pub fn into_crdt(self) -> CRDTSetParam {
			match self {
				SetParam::SetLocalId(v) => CRDTSetParam::SetLocalId(v),
				SetParam::SetCasId(v) => CRDTSetParam::SetCasId(v),
				SetParam::SetSizeInBytes(v) => CRDTSetParam::SetSizeInBytes(v),
			}
		}
	}

	impl Into<crate::prisma::file::SetParam> for SetParam {
		fn into(self) -> crate::prisma::file::SetParam {
			match self {
				SetParam::SetLocalId(v) => crate::prisma::file::local_id::set(v),
				SetParam::SetCasId(v) => crate::prisma::file::cas_id::set(v),
				SetParam::SetSizeInBytes(v) => crate::prisma::file::size_in_bytes::set(v),
			}
		}
	}

	impl From<crate::prisma::file::size_in_bytes::Set> for SetParam {
		fn from(v: crate::prisma::file::size_in_bytes::Set) -> Self {
			SetParam::SetSizeInBytes(v.0)
		}
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum CRDTSetParam {
		#[serde(rename = "local_id")]
		SetLocalId(i32),
		#[serde(rename = "cas_id")]
		SetCasId(Vec<u8>),
		#[serde(rename = "size_in_bytes")]
		SetSizeInBytes(i32),
	}

	impl Into<crate::prisma::file::SetParam> for CRDTSetParam {
		fn into(self) -> crate::prisma::file::SetParam {
			match self {
				CRDTSetParam::SetLocalId(v) => crate::prisma::file::local_id::set(v),
				CRDTSetParam::SetCasId(v) => crate::prisma::file::cas_id::set(v),
				CRDTSetParam::SetSizeInBytes(v) => crate::prisma::file::size_in_bytes::set(v),
			}
		}
	}

	pub struct Create<'a> {
		client: &'a super::_prisma::PrismaCRDTClient,
		set_params: CreateParams,
		with_params: Vec<crate::prisma::file::WithParam>,
	}

	#[derive(Clone)]
	pub(super) struct CreateParams {
		cas_id: Vec<u8>,
		_params: Vec<SetParam>,
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub(super) struct CRDTCreateParams {
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		pub _params: Vec<CRDTSetParam>,
	}

	impl<'a> Create<'a> {
		pub fn with(mut self, param: impl Into<crate::prisma::file::WithParam>) -> Self {
			self.with_params.push(param.into());
			self
		}

		pub async fn exec(self) -> Result<crate::prisma::file::Data, crate::prisma::QueryError> {
			let Self {
				client, set_params, ..
			} = self;

			// THIS SHOULD BE IN A TRANSACTION
			let sync_id = set_params.cas_id.clone();

			// Create record locally
			let res = {
				let CreateParams {
					cas_id: field_0,
					_params,
				} = set_params.clone();

				client
					.client
					.file()
					.create(
						field_0,
						_params.into_iter().map(Into::into).collect::<Vec<_>>(),
					)
					.exec()
					.await
			};

			// Write + send create operation
			client
				._create_operation(prisma_crdt::CRDTOperationType::shared(
					sync_id.clone(),
					"File",
					prisma_crdt::SharedOperationData::create_atomic(),
				))
				.await;

			let CreateParams { _params, .. } = set_params;
			// Write + send atomic update operations
			for param in _params.into_iter() {
				let crdt_param: CRDTSetParam = param.into_crdt();

				let param_map = match ::serde_json::to_value(crdt_param).unwrap() {
					::serde_json::Value::Object(v) => v,
					_ => unreachable!(),
				};

				for (key, value) in param_map {
					client
						._create_operation(prisma_crdt::CRDTOperationType::shared(
							sync_id.clone(),
							"File",
							prisma_crdt::SharedOperationData::update(key, value),
						))
						.await;
				}
			}

			res
		}
	}

	pub struct Actions<'a> {
		pub(super) client: &'a super::_prisma::PrismaCRDTClient,
	}

	impl<'a> Actions<'a> {
		pub fn create(
			self,
			cas_id: Vec<u8>,
			mut _params: Vec<super::file::SetParam>,
		) -> Create<'a> {
			Create {
				client: self.client,
				set_params: CreateParams {
					cas_id,
					_params: _params.into_iter().map(Into::into).collect(),
				},
				with_params: vec![],
			}
		}

		pub fn find_unique(
			self,
			param: crate::prisma::file::UniqueWhereParam,
		) -> crate::prisma::file::FindUnique<'a> {
			self.client.client.file().find_unique(param)
		}

		pub fn find_many(
			self,
			params: Vec<crate::prisma::file::WhereParam>,
		) -> crate::prisma::file::FindMany<'a> {
			self.client.client.file().find_many(params)
		}
	}
}
pub mod tag {
	use super::*;
	pub use crate::prisma::tag::*;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum SetParam {
		SetId(Vec<u8>),
		SetName(String),
	}

	impl SetParam {
		pub fn into_crdt(self) -> CRDTSetParam {
			match self {
				SetParam::SetId(v) => CRDTSetParam::SetId(v),
				SetParam::SetName(v) => CRDTSetParam::SetName(v),
			}
		}
	}

	impl Into<crate::prisma::tag::SetParam> for SetParam {
		fn into(self) -> crate::prisma::tag::SetParam {
			match self {
				SetParam::SetId(v) => crate::prisma::tag::id::set(v),
				SetParam::SetName(v) => crate::prisma::tag::name::set(v),
			}
		}
	}

	impl From<crate::prisma::tag::name::Set> for SetParam {
		fn from(param: crate::prisma::tag::name::Set) -> Self {
			SetParam::SetName(param.0)
		}
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum CRDTSetParam {
		#[serde(rename = "pub_id")]
		SetId(Vec<u8>),
		#[serde(rename = "name")]
		SetName(String),
	}

	impl Into<crate::prisma::tag::SetParam> for CRDTSetParam {
		fn into(self) -> crate::prisma::tag::SetParam {
			match self {
				CRDTSetParam::SetId(v) => crate::prisma::tag::id::set(v),
				CRDTSetParam::SetName(v) => crate::prisma::tag::name::set(v),
			}
		}
	}

	pub struct Create<'a> {
		client: &'a _prisma::PrismaCRDTClient,
		set_params: CreateParams,
		with_params: Vec<crate::prisma::tag::WithParam>,
	}

	#[derive(Clone)]
	pub(super) struct CreateParams {
		id: Vec<u8>,
		name: String,
		_params: Vec<SetParam>,
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub(super) struct CRDTCreateParams {
		pub id: Vec<u8>,
		pub name: String,
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		pub _params: Vec<CRDTSetParam>,
	}

	impl<'a> Create<'a> {
		pub fn with(mut self, param: crate::prisma::tag::WithParam) -> Self {
			self.with_params.push(param);
			self
		}

		pub async fn exec(self) -> Result<crate::prisma::tag::Data, crate::prisma::QueryError> {
			let Self {
				client, set_params, ..
			} = self;

			let sync_id = set_params.id.clone();

			// Create record locally
			let res = {
				let CreateParams {
					id: field_0,
					name: field_1,
					_params,
				} = set_params.clone();

				client
					.client
					.tag()
					.create(
						field_0,
						field_1,
						_params.into_iter().map(Into::into).collect::<Vec<_>>(),
					)
					.exec()
					.await
			};

			let op = {
				let CreateParams {
					id: field_0,
					name: field_1,
					_params,
				} = set_params;

				let set_params_map = match serde_json::to_value(CRDTCreateParams {
					id: field_0,
					name: field_1,
					_params: _params.into_iter().map(|v| v.into_crdt()).collect(),
				})
				.unwrap()
				{
					serde_json::Value::Object(v) => v,
					_ => unreachable!(),
				};

				client
					._create_operation(prisma_crdt::CRDTOperationType::shared(
						sync_id.clone(),
						"Tag",
						prisma_crdt::SharedOperationData::create_unique(set_params_map),
					))
					.await;
			};

			res
		}
	}

	pub struct Delete<'a> {
		client: &'a super::_prisma::PrismaCRDTClient,
		r#where: crate::prisma::tag::UniqueWhereParam,
		with_params: Vec<crate::prisma::tag::WithParam>,
	}

	impl<'a> Delete<'a> {
		pub fn with(mut self, param: crate::prisma::tag::WithParam) -> Self {
			self.with_params.push(param);
			self
		}

		pub async fn exec(
			self,
		) -> Result<Option<crate::prisma::tag::Data>, crate::prisma::QueryError> {
			let Self {
				client, r#where, .. // TODO: honor with params
			} = self;

			let res = client.client.tag().delete(r#where).exec().await;

			if let Ok(Some(res)) = &res {
				let sync_id = res.id.clone();

				self.client
					._create_operation(::prisma_crdt::CRDTOperationType::shared(
						sync_id,
						"Tag",
						::prisma_crdt::SharedOperationData::delete(),
					))
					.await;
			}

			res
		}
	}

	pub struct Actions<'a> {
		pub(super) client: &'a _prisma::PrismaCRDTClient,
	}

	impl<'a> Actions<'a> {
		pub fn create(
			self,
			id: Vec<u8>,
			name: String,
			mut _params: Vec<super::tag::SetParam>,
		) -> Create<'a> {
			Create {
				client: self.client,
				set_params: CreateParams {
					id,
					name,
					_params: _params.into_iter().map(Into::into).collect(),
				},
				with_params: vec![],
			}
		}

		pub fn find_unique(
			self,
			param: crate::prisma::tag::UniqueWhereParam,
		) -> crate::prisma::tag::FindUnique<'a> {
			self.client.client.tag().find_unique(param)
		}

		pub fn find_many(
			self,
			params: Vec<crate::prisma::tag::WhereParam>,
		) -> crate::prisma::tag::FindMany<'a> {
			self.client.client.tag().find_many(params)
		}

		pub fn delete(
			self,
			param: crate::prisma::tag::UniqueWhereParam,
		) -> crate::prisma::tag::Delete<'a> {
			self.client.client.tag().delete(param)
		}
	}
}
pub mod node {
	use super::*;
	pub use crate::prisma::node::*;
}
pub mod tag_on_file {
	use super::{file_path::CRDTUpdateParams, *};
	pub use crate::prisma::tag_on_file::*;

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum SetParam {
		SetTagId(i32),
		SetFileId(i32),
	}

	// impl SetParam {
	// 	pub async fn into_crdt(self, client: &_prisma::PrismaCRDTClient) -> CRDTSetParam {
	// 		match self {
	// 			SetParam::SetTagId(v) => CRDTSetParam::SetLocalId(v),
	// 			SetParam::SetFileId(v) => CRDTSetParam::SetCasId(v),
	// 		}
	// 	}
	// }

	impl Into<crate::prisma::tag_on_file::SetParam> for SetParam {
		fn into(self) -> crate::prisma::tag_on_file::SetParam {
			match self {
				SetParam::SetTagId(v) => crate::prisma::tag_on_file::tag_id::set(v),
				SetParam::SetFileId(v) => crate::prisma::tag_on_file::file_id::set(v),
			}
		}
	}

	impl From<crate::prisma::tag_on_file::tag_id::Set> for SetParam {
		fn from(v: crate::prisma::tag_on_file::tag_id::Set) -> Self {
			SetParam::SetTagId(v.0)
		}
	}

	impl From<crate::prisma::tag_on_file::file_id::Set> for SetParam {
		fn from(v: crate::prisma::tag_on_file::file_id::Set) -> Self {
			SetParam::SetFileId(v.0)
		}
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum CRDTSetParam {
		#[serde(rename = "tag_id")]
		SetTagId(Vec<u8>),
		#[serde(rename = "file_id")]
		SetFileId(Vec<u8>),
	}

	impl Into<crate::prisma::tag_on_file::SetParam> for CRDTSetParam {
		fn into(self) -> prisma::tag_on_file::SetParam {
			match self {
				CRDTSetParam::SetTagId(v) => {
					crate::prisma::tag_on_file::tag::link(crate::prisma::tag::id::equals(v))
				}
				CRDTSetParam::SetFileId(v) => {
					crate::prisma::tag_on_file::file::link(crate::prisma::file::cas_id::equals(v))
				}
			}
		}
	}

	pub struct Create<'a> {
		client: &'a _prisma::PrismaCRDTClient,
		set_params: CreateParams,
		with_params: Vec<crate::prisma::file::WithParam>,
	}

	#[derive(Clone)]
	pub(super) struct CreateParams {
		tag_id: i32,
		file_id: i32,
		_params: Vec<SetParam>,
	}

	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub(super) struct CRDTCreateParams {
		#[serde(default, skip_serializing_if = "Vec::is_empty", rename = "_")]
		pub _params: Vec<CRDTSetParam>,
	}

	impl<'a> Create<'a> {
		pub fn with(mut self, param: impl Into<crate::prisma::file::WithParam>) -> Self {
			self.with_params.push(param.into());
			self
		}

		pub async fn exec(
			self,
		) -> Result<crate::prisma::tag_on_file::Data, crate::prisma::QueryError> {
			let Self {
				client, set_params, ..
			} = self;

			let relation_item = {
				client
					.client
					.file()
					.find_unique(crate::prisma::file::local_id::equals(set_params.file_id))
					.exec()
					.await
					.unwrap()
					.unwrap()
					.cas_id
			};

			let relation_group = {
				client
					.client
					.tag()
					.find_unique(crate::prisma::tag::local_id::equals(set_params.tag_id))
					.exec()
					.await
					.unwrap()
					.unwrap()
					.id
			};

			// Create record locally
			let res = {
				let CreateParams {
					tag_id: field_0,
					file_id: field_1,
					_params,
				} = set_params.clone();

				client
					.client
					.tag_on_file()
					.create(
						crate::prisma::tag::local_id::equals(field_0),
						crate::prisma::file::local_id::equals(field_1),
						_params.into_iter().map(Into::into).collect::<Vec<_>>(),
					)
					.exec()
					.await
			};

			// Write + send create operation
			client
				._create_operation(prisma_crdt::CRDTOperationType::relation(
					"TagOnFile",
					relation_item,
					relation_group,
					prisma_crdt::RelationOperationData::create(),
				))
				.await;

			// let CreateParams { _params, .. } = set_params;
			// // Write + send atomic update operations
			// for param in _params.into_iter() {
			// 	let crdt_param: CRDTSetParam = param.into_crdt();

			// 	let param_map = match ::serde_json::to_value(crdt_param).unwrap() {
			// 		::serde_json::Value::Object(v) => v,
			// 		_ => unreachable!(),
			// 	};

			// 	for (key, value) in param_map {
			// 		client
			// 			._send_operation(prisma_crdt::CRDTOperation::new(
			// 				self.client.node_id.clone(),
			// 				uhlc::NTP64(0),
			// 				prisma_crdt::CRDTOperationType::shared(
			// 					sync_id.clone(),
			// 					"File",
			// 					prisma_crdt::SharedOperationData::update(key, value),
			// 				),
			// 			))
			// 			.await;
			// 	}
			// }

			res
		}
	}

	pub struct Actions<'a> {
		pub(super) client: &'a _prisma::PrismaCRDTClient,
	}

	impl<'a> Actions<'a> {
		pub fn create(
			self,
			tag_id: i32,
			file_id: i32,
			mut _params: Vec<super::tag_on_file::SetParam>,
		) -> Create<'a> {
			Create {
				client: self.client,
				set_params: CreateParams {
					tag_id,
					file_id,
					_params: _params.into_iter().map(Into::into).collect(),
				},
				with_params: vec![],
			}
		}

		pub fn find_unique(
			self,
			param: crate::prisma::tag_on_file::UniqueWhereParam,
		) -> crate::prisma::tag_on_file::FindUnique<'a> {
			self.client.client.tag_on_file().find_unique(param)
		}

		pub fn find_many(
			self,
			params: Vec<crate::prisma::tag_on_file::WhereParam>,
		) -> crate::prisma::tag_on_file::FindMany<'a> {
			self.client.client.tag_on_file().find_many(params)
		}
	}
}
pub mod _prisma {
	use prisma_crdt::CRDTOperation;
	use tokio::sync::mpsc::error::SendError;

	use super::*;

	pub struct PrismaCRDTClient {
		pub(super) client: prisma::PrismaClient,
		pub node_id: Vec<u8>,
		pub node_local_id: i32,
		operation_sender: tokio::sync::mpsc::Sender<prisma_crdt::CRDTOperation>,
	}

	impl PrismaCRDTClient {
		pub(super) fn _new(
			client: prisma::PrismaClient,
			(node_id, node_local_id): (Vec<u8>, i32),
			operation_sender: tokio::sync::mpsc::Sender<prisma_crdt::CRDTOperation>,
		) -> Self {
			Self {
				client,
				operation_sender,
				node_id,
				node_local_id,
			}
		}

		pub async fn _execute_operation(&self, op: ::prisma_crdt::CRDTOperation) {
			let prisma_crdt::CRDTOperation {
				node,
				timestamp,
				typ,
			} = op;

			match typ {
				prisma_crdt::CRDTOperationType::Shared(op) => {
					let prisma_crdt::SharedOperation {
						record_id,
						model,
						data,
					} = op;

					match model.as_str() {
						"File" => match data {
							prisma_crdt::SharedOperationData::Create(data) => {
								match data {
									prisma_crdt::SharedOperationCreateData::Atomic => {}
									_ => unreachable!(),
								};

								self.client
									.file()
									.upsert(
										super::file::cas_id::equals(record_id.clone()),
										(record_id, vec![]),
										vec![],
									)
									.exec()
									.await
									.unwrap();
							}

							prisma_crdt::SharedOperationData::Update { field, value } => {
								self.client
									.file()
									.update(
										super::file::cas_id::equals(record_id.clone()),
										vec![serde_json::from_value::<super::file::CRDTSetParam>(
											vec![(field, value)].into_iter().collect(),
										)
										.unwrap()
										.into()],
									)
									.exec()
									.await
									.unwrap();
							}
							_ => todo!(),
						},
						"Tag" => match data {
							prisma_crdt::SharedOperationData::Create(data) => {
								let data = match data {
									::prisma_crdt::SharedOperationCreateData::Unique(data) => data,
									_ => unreachable!(),
								};

								let tag::CRDTCreateParams {
									id: field_0,
									name: field_1,
									_params,
								} = ::serde_json::from_value(serde_json::Value::Object(data)).unwrap();

								self.client
									.tag()
									.create(
										field_0,
										field_1,
										_params.into_iter().map(Into::into).collect(),
									)
									.exec()
									.await
									.unwrap();
							}
							_ => todo!(),
						},
						_ => {}
					}
				}
				prisma_crdt::CRDTOperationType::Owned(op) => {
					let prisma_crdt::OwnedOperation { model, data } = op;

					match model.as_str() {
						"FilePath" => {
							for data in data {
								match data {
									prisma_crdt::OwnedOperationData::Create(create_args) => {
										let file_path::CRDTCreateParams {
											id: field_0,
											location_id: field_1,
											name: field_2,
											_params,
										} = ::serde_json::from_value(serde_json::Value::Object(
											create_args,
										))
										.unwrap();

										self.client
											.file_path()
											.create(
												field_0,
												prisma::location::id::equals(field_1),
												field_2,
												_params.into_iter().map(Into::into).collect(),
											)
											.exec()
											.await
											.unwrap();
									}
									prisma_crdt::OwnedOperationData::Update(update_args) => {
										let file_path::CRDTUpdateParams {
											location_id: field_0,
											id: field_1,
											_params,
										} = serde_json::from_value(serde_json::Value::Object(
											update_args,
										))
										.unwrap();

										let field_0 = {
											self.client
												.location()
												.find_unique(crate::prisma::location::id::equals(
													field_0,
												))
												.exec()
												.await
												.unwrap()
												.unwrap()
												.local_id
										};

										self.client
											.file_path()
											.update(
												prisma::file_path::location_id_id(field_0, field_1),
												_params.into_iter().map(Into::into).collect(),
											)
											.exec()
											.await
											.unwrap()
											.unwrap();
									}
									_ => todo!(),
								}
							}
						}
						"Location" => {
							for data in data {
								match data {
									prisma_crdt::OwnedOperationData::Create(create_args) => {
										let create_args: location::CRDTCreateParams =
											serde_json::from_value(serde_json::Value::Object(
												create_args,
											))
											.unwrap();

										let location::CRDTCreateParams {
											id: field_0,
											name: field_1,
											_params,
										} = create_args;

										self.client
											.location()
											.create(
												field_0,
												prisma::node::id::equals(node.clone()),
												field_1,
												_params.into_iter().map(Into::into).collect(),
											)
											.exec()
											.await
											.unwrap();
									}
									_ => todo!(),
								}
							}
						}
						_ => todo!(),
					}
				}
				prisma_crdt::CRDTOperationType::Relation(op) => {
					let prisma_crdt::RelationOperation {
						relation,
						relation_item,
						relation_group,
						data,
					} = op;

					match relation.as_str() {
						"TagOnFile" => {
							let field_0 = self
								.client
								.tag()
								.find_unique(prisma::tag::id::equals(relation_item.clone()))
								.exec()
								.await
								.unwrap()
								.unwrap()
								.local_id;

							let field_1 = self
								.client
								.file()
								.find_unique(prisma::file::cas_id::equals(relation_group.clone()))
								.exec()
								.await
								.unwrap()
								.unwrap()
								.local_id;

							match data {
								prisma_crdt::RelationOperationData::Create => {
									self.client
										.tag_on_file()
										.upsert(
											crate::prisma::tag_on_file::tag_id_file_id(
												field_0, field_1,
											),
											(
												crate::prisma::tag::id::equals(relation_group),
												crate::prisma::file::cas_id::equals(relation_item),
												vec![],
											),
											vec![],
										)
										.exec()
										.await;
								}
								_ => todo!(),
							}
						}
						_ => todo!(),
					}
				}
			}
		}

		pub async fn _create_operation(&self, typ: ::prisma_crdt::CRDTOperationType) {
			let timestamp = ::uhlc::NTP64(0); // TODO: actual timestamps

			let timestamp_bytes = vec![0];

			use prisma_crdt::*;

			match &typ {
				CRDTOperationType::Shared(SharedOperation {
					record_id,
					model,
					data,
				}) => {
					let (kind, data) = match data {
						SharedOperationData::Create(typ) => {
							("c".to_string(), serde_json::to_vec(typ).unwrap())
						}
						SharedOperationData::Update { field, value } => {
							("u".to_string() + field, serde_json::to_vec(value).unwrap())
						}
						SharedOperationData::Delete => ("d".to_string(), vec![]),
					};

					self.client
						.shared_operation()
						.create(
							timestamp_bytes,
							record_id.clone(),
							kind,
							model.to_string(),
							data,
							crate::prisma::node::local_id::equals(self.node_local_id),
							vec![],
						)
						.exec()
						.await;
				}
				CRDTOperationType::Owned(op) => {
					self.client
						.owned_operation()
						.create(
							timestamp_bytes,
							serde_json::to_vec(op).unwrap(),
							crate::prisma::node::local_id::equals(self.node_local_id),
							vec![],
						)
						.exec()
						.await;
				}
				CRDTOperationType::Relation(RelationOperation {
					relation,
					relation_item,
					relation_group,
					data,
				}) => {
					let (kind, data) = match data {
						RelationOperationData::Create => ("c".to_string(), vec![]),
						RelationOperationData::Update { field, value } => {
							("u".to_string() + field, serde_json::to_vec(value).unwrap())
						}
						RelationOperationData::Delete => ("d".to_string(), vec![]),
					};

					self.client
						.relation_operation()
						.create(
							timestamp_bytes,
							relation.to_string(),
							relation_item.clone(),
							relation_group.clone(),
							kind,
							data,
							crate::prisma::node::local_id::equals(self.node_local_id),
							vec![],
						)
						.exec()
						.await;
				}
			}

			let op = CRDTOperation::new(self.node_id.clone(), timestamp, typ);

			self.operation_sender.send(op).await;
		}

		pub fn node(&self) -> prisma::node::Actions {
			self.client.node()
		}

		pub fn location(&self) -> location::Actions {
			location::Actions { client: self }
		}

		pub fn file_path(&self) -> file_path::Actions {
			file_path::Actions { client: self }
		}

		pub fn file(&self) -> file::Actions {
			file::Actions { client: self }
		}

		pub fn tag(&self) -> tag::Actions {
			tag::Actions { client: self }
		}

		pub fn tag_on_file(&self) -> tag_on_file::Actions {
			tag_on_file::Actions { client: self }
		}
	}
}
pub use _prisma::*;
